use indexmap::IndexMap;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
   models::orders::{OrderDetailModel, OrderModel, OrderStatus},
   routes::app_error::AppError,
   schemas::{
      image::get_img_path_by_id,
      order::{BookAtPurchase, Order, OrderDetail},
   },
};

#[derive(Clone)]
pub struct OrderRepo {
   pool: PgPool,
}

/// For User
impl OrderRepo {
   pub fn new(pool: PgPool) -> Self {
      Self { pool }
   }

   pub async fn get_order_detail_by_id(
      &self,
      user_id: Uuid,
      order_id: Uuid,
   ) -> Result<OrderDetail, AppError> {
      let items_detail = sqlx::query_as::<_, OrderDetailModel>(
         r#"
            SELECT
	            o.id, o.created_at, o.updated_at, o.total_price, o.order_status, oi.book_id, b.title,
					b.genre, b.image_id, oi.quantity, oi.price_at_purchase
            FROM order_items oi
            JOIN orders o ON o.id = oi.order_id
            JOIN books b ON b.id = oi.book_id
            WHERE o.user_id = $1 AND o.order_id = $2
            ORDER BY oi.updated_at DESC
         "#,
      )
      .bind(user_id)
      .bind(order_id)
      .fetch_all(&self.pool)
      .await?;

      Self::convert_order_items(items_detail)
         .pop()
         .ok_or(AppError::NotFound("invalid order item id".to_string()))
   }

   // get all orders with detail order by latest to earliest
   pub async fn get_all_orders_detail(&self, user_id: Uuid) -> Result<Vec<OrderDetail>, AppError> {
      let items_detail = sqlx::query_as::<_, OrderDetailModel>(
         r#"
            SELECT
	            o.id, o.created_at, o.updated_at, o.total_price, o.order_status, oi.book_id, b.title,
					b.genre, b.image_id, oi.quantity, oi.price_at_purchase
            FROM order_items oi
            JOIN orders o ON o.id = oi.order_id
            JOIN books b ON b.id = oi.book_id
            WHERE o.user_id = $1
            ORDER BY o.updated_at DESC
         "#,
      )
      .bind(user_id)
      .fetch_all(&self.pool)
      .await?;

      if items_detail.is_empty() {
         return Ok(vec![]);
      }

      Ok(Self::convert_order_items(items_detail))
   }

   // get all orders with detail (only pending status) order by earliest
   pub async fn get_pending_orders_detail(
      &self,
      user_id: Uuid,
   ) -> Result<Vec<OrderDetail>, AppError> {
      let items_detail = sqlx::query_as::<_, OrderDetailModel>(
         r#"
            SELECT
	            o.id, o.created_at, o.updated_at, o.total_price, o.order_status, oi.book_id, b.title,
					b.genre, b.image_id, oi.quantity, oi.price_at_purchase
            FROM order_items oi
            JOIN orders o ON o.id = oi.order_id
            JOIN books b ON b.id = oi.book_id
            WHERE o.user_id = $1 AND o.order_status = $2
            ORDER BY o.updated_at
         "#,
      )
      .bind(user_id)
      .bind(OrderStatus::Pending)
      .fetch_all(&self.pool)
      .await?;

      if items_detail.is_empty() {
         return Ok(vec![]);
      }

      Ok(Self::convert_order_items(items_detail))
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

   fn convert_order_items(models: Vec<OrderDetailModel>) -> Vec<OrderDetail> {
      // Use IdexMap to preserves order
      let mut order_map: IndexMap<Uuid, OrderDetail> = IndexMap::new();

      for model in models {
         let book = BookAtPurchase {
            book_id: model.book_id,
            title: model.title,
            genre: model.genre,
            img_path: get_img_path_by_id(model.image_id),
            quantity: model.quantity,
            price_at_purchase: model.price_at_purchase,
         };

         order_map
            .entry(model.id)
            .or_insert_with(|| OrderDetail {
               id: model.id,
               total_price: model.total_price,
               order_status: model.order_status.to_string(),
               created_at: model.created_at,
               updated_at: model.updated_at,
               items: Vec::new(),
            })
            .items
            .push(book);
      }

      order_map.into_values().collect()
   }
}

// For admin only
impl OrderRepo {
   pub async fn get_all_order(&self) -> Result<Vec<Order>, AppError> {
      let order_models = sqlx::query_as::<_, OrderModel>(
         r#"
            SELECT
               id, user_id, total_price, order_status, created_at, updated_at
            FROM orders
            WHERE user_id = $1
         "#,
      )
      .fetch_all(&self.pool)
      .await?;

      if order_models.is_empty() {
         return Ok(vec![]);
      }

      Ok(order_models.into_iter().map(|order| Order::from(order)).collect())
   }
}
