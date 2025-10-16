use sqlx::PgPool;
use uuid::Uuid;

use crate::{
   models::orders::{OrderItemDetail, OrderModel},
   routes::app_error::AppError,
};

#[derive(Clone)]
pub struct OrderRepo {
   pool: PgPool,
}

impl OrderRepo {
   pub fn new(pool: PgPool) -> Self {
      Self { pool }
   }

   pub async fn get_user_order(&self, user_id: Uuid) -> Result<Vec<OrderModel>, AppError> {
      let orders = sqlx::query_as::<_, OrderModel>(
         r#"
            SELECT
               id, user_id, total_price, order_status, created_at, updated_at
            FROM orders
            WHERE user_id = $1
         "#,
      )
      .bind(user_id)
      .fetch_all(&self.pool)
      .await?;

      if orders.is_empty() {
         return Ok(vec![]);
      }

      Ok(orders)
   }

   pub async fn get_order_detail(
      &self,
      user_id: Uuid,
      order_id: Uuid,
   ) -> Result<Vec<OrderItemDetail>, AppError> {
      let items_detail = sqlx::query_as::<_, OrderItemDetail>(
         r#"
            SELECT
               oi.id, oi.book_id, b.genre, b.description, b.image_id, oi.quantity, oi.price_at_purchase
            FROM order_items oi
            JOIN orders o ON o.id = oi.order_id
            JOIN books b ON b.id = oi.book_id
            WHERE o.user_id = $1 AND oi.order_id = $2
         "#,
      )
      .bind(user_id)
      .bind(order_id)
      .fetch_all(&self.pool)
      .await?;

      if items_detail.is_empty() {
         return Ok(vec![]);
      }

      Ok(items_detail)
   }

   pub async fn get_order_item_detail(
      &self,
      user_id: Uuid,
      order_item_id: Uuid,
   ) -> Result<OrderItemDetail, AppError> {
      let item_detail = sqlx::query_as::<_, OrderItemDetail>(
         r#"
            SELECT
               oi.id, oi.book_id, b.genre, b.description, b.image_id, oi.quantity, oi.price_at_purchase
            FROM order_items oi
            JOIN orders o ON o.id = oi.order_id
            JOIN books b ON b.id = oi.book_id
            WHERE o.user_id = $1 AND oi.id = $2
         "#,
      )
      .bind(user_id)
      .bind(order_item_id)
      .fetch_optional(&self.pool)
      .await?
      .ok_or(AppError::NotFound("invalid order item id".to_string()))?;

      Ok(item_detail)
   }

   pub async fn pay_order(&self, user_id: Uuid, order_id: Uuid) -> Result<(), AppError> {
      // Create transaction
      let mut tx = self.pool.begin().await?;

      // 1. Validate order status
      sqlx::query(
         r#"
            SELECT 1 FROM orders
            WHERE user_id = $1 AND id = $2 AND order_status = 'pending'
         "#,
      )
      .bind(user_id)
      .bind(order_id)
      .fetch_optional(&mut *tx)
      .await?
      .ok_or(AppError::Conflict(
         "Order status are not available to pay".to_string(),
      ))?;

      // 2. Validate stock
      let insuf_item_ids: Vec<Uuid> = sqlx::query_scalar(
         r#"
            SELECT oi.id
            FROM order_items oi
            JOIN orders o ON o.id = oi.order_id
            JOIN books b ON b.id = oi.book_id
            WHERE o.user_id = $1 AND o.id = $2 AND oi.quantity > b.available
         "#,
      )
      .bind(user_id)
      .bind(order_id)
      .fetch_all(&mut *tx)
      .await?;

      if insuf_item_ids.len() > 0 {
         return Err(AppError::Conflict(
            "Insufficient stock for some items in order".to_string(),
         ));
      }

      // 3. Decrease quantity in stock
      sqlx::query(
         r#"
            UPDATE books b
            SET available = b.available - oi.quantity
            FROM order_items oi
            JOIN orders o ON o.id = oi.order_id
            WHERE o.user_id = $1 AND o.id = $2 AND oi.book_id = b.id
         "#,
      )
      .bind(user_id)
      .bind(order_id)
      .execute(&mut *tx)
      .await?;

      // 4. Update order status
      sqlx::query(
         r#"
            UPDATE orders SET
               order_status = 'paid'
            WHERE user_id = $1 AND id = $2
         "#,
      )
      .bind(user_id)
      .bind(order_id)
      .execute(&mut *tx)
      .await?;

      // Commit transaction
      tx.commit().await?;

      Ok(())
   }

   pub async fn cancel_order(&self, user_id: Uuid, order_id: Uuid) -> Result<(), AppError> {
      // Create transaction
      let mut tx = self.pool.begin().await?;

      // 1. Validate order status
      sqlx::query(
         r#"
            SELECT 1 FROM orders
            WHERE user_id = $1 AND id = $2 AND order_status = 'pending'
         "#,
      )
      .bind(user_id)
      .bind(order_id)
      .fetch_optional(&mut *tx)
      .await?
      .ok_or(AppError::Conflict(
         "Order status are not available to cancel".to_string(),
      ))?;

      // 2. Delete order items (We already validated user_id and order_id in earlier)
      sqlx::query(
         r#"
            DELETE FROM order_items
            WHERE order_id = $1
         "#,
      )
      .bind(order_id)
      .execute(&mut *tx)
      .await?;
      
      // 3. Update order status
      sqlx::query(
         r#"
            UPDATE orders SET
               order_status = 'cancelled'
            WHERE user_id = $1 AND id = $2
         "#,
      )
      .bind(user_id)
      .bind(order_id)
      .execute(&mut *tx)
      .await?;

      // Commit transaction
      tx.commit().await?;

      Ok(())
   }
}
