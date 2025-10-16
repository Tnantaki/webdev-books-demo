use sqlx::PgPool;
use uuid::Uuid;

use crate::{
   models::cart_items::CartItemModel,
   routes::app_error::AppError,
   schemas::cart::{AddCartItem, EditCartItem},
};

#[derive(Clone)]
pub struct CartItemRepo {
   pool: PgPool,
}

impl CartItemRepo {
   pub fn new(pool: PgPool) -> Self {
      Self { pool }
   }

   pub async fn add_to_cart(
      &self,
      user_id: Uuid,
      add_cart: AddCartItem,
   ) -> Result<CartItemModel, AppError> {
      let cart_item = sqlx::query_as::<_, CartItemModel>(
         r#"
            INSERT INTO cart_items
               (user_id, book_id, quantity)
            VALUES ($1, $2, $3)
            RETURNING
               id, user_id, book_id, quantity, created_at, updated_at
         "#,
      )
      .bind(user_id)
      .bind(add_cart.book_id)
      .bind(add_cart.quantity)
      .fetch_one(&self.pool)
      .await?;

      Ok(cart_item)
   }

   pub async fn get_cart_items(&self, user_id: Uuid) -> Result<Vec<CartItemModel>, AppError> {
      let cart = sqlx::query_as::<_, CartItemModel>(
         r#"
            SELECT id, user_id, book_id, quantity, created_at, updated_at
            FROM cart_items
            WHERE user_id = $1
         "#,
      )
      .bind(user_id)
      .fetch_all(&self.pool)
      .await?;

      if cart.is_empty() {
         return Ok(vec![]);
      }
      Ok(cart)
   }

   pub async fn edit_cart_item(
      &self,
      user_id: Uuid,
      id: Uuid,
      item: EditCartItem,
   ) -> Result<CartItemModel, AppError> {
      let cart_item = sqlx::query_as::<_, CartItemModel>(
         r#"
            UPDATE cart_items
            SET quantity = $1
            WHERE user_id = $2 AND id = $3
            RETURNING
               id, user_id, book_id, quantity, created_at, updated_at
         "#,
      )
      .bind(item.quantity)
      .bind(user_id)
      .bind(id)
      .fetch_optional(&self.pool)
      .await?
      .ok_or(AppError::NotFound(
         "invalid user id or cart item id".to_string(),
      ))?;

      Ok(cart_item)
   }

   pub async fn remove_from_cart(&self, user_id: Uuid, id: Uuid) -> Result<(), AppError> {
      let row_affects = sqlx::query(
         r#"
            DELETE FROM cart_items
            WHERE user_id = $1 AND id = $2
         "#,
      )
      .bind(user_id)
      .bind(id)
      .execute(&self.pool)
      .await?
      .rows_affected();

      if row_affects == 0 {
         Err(AppError::NotFound(
            "invalid user id or cart item id".to_string(),
         ))
      } else {
         Ok(())
      }
   }
}
