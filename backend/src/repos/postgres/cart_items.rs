use std::collections::HashMap;

use rust_decimal::{Decimal, dec};
use sqlx::PgPool;
use uuid::Uuid;

const MIN_FREE_SHIPPING: Decimal = dec!(50); // TODO: must store in database (site_settings)

use crate::{
   models::{books::BookModel, cart_items::CartItemModel, orders::OrderModel},
   routes::app_error::AppError,
   schemas::cart::{AddCartItem, Cart, CartItem, EditCartItem},
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
            INSERT INTO cart_items (user_id, book_id, quantity)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, book_id)
            DO UPDATE SET
            	quantity = cart_items.quantity + EXCLUDED.quantity
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

   pub async fn get_cart_items(&self, user_id: Uuid) -> Result<Cart, AppError> {
      let cart_item = sqlx::query_as::<_, CartItemModel>(
         r#"
            SELECT id, user_id, book_id, quantity, created_at, updated_at
            FROM cart_items
            WHERE user_id = $1
            ORDER BY created_at
         "#,
      )
      .bind(user_id)
      .fetch_all(&self.pool)
      .await?;

      if cart_item.is_empty() {
         return Ok(Cart {
            items: vec![],
            total_price: dec!(0),
            shipping_price: dec!(0),
         });
      }

      // Get all book detail
      let book_ids: Vec<Uuid> = cart_item.iter().map(|item| item.book_id).collect();
      let books = sqlx::query_as::<_, BookModel>(
         r#"
	         SELECT
	            id, title, genre, description, price, available, image_id,
	            average_rating, total_ratings, created_at, updated_at
	         FROM books
				WHERE id = ANY($1)
         "#,
      )
      .bind(book_ids)
      .fetch_all(&self.pool)
      .await?;

      // Create a map for quick lookup
      let book_map: HashMap<Uuid, BookModel> =
         books.into_iter().map(|book| (book.id, book)).collect();

      // Combine cart items with book details
      let items: Vec<CartItem> = cart_item
         .into_iter()
         .filter_map(|item| {
            book_map.get(&item.book_id).map(|book| CartItem {
               id: item.id,
               quantity: item.quantity,
               updated_at: item.updated_at,
               book: book.clone().into(),
            })
         })
         .collect();

      let total_price: Decimal = items
         .iter()
         .map(|item| item.book.price * Decimal::new(item.quantity as i64, 0))
         .sum();

      let shipping_price = match total_price > MIN_FREE_SHIPPING {
         true => dec!(0),
         false => dec!(50),
      };

      Ok(Cart {
         items,
         total_price,
         shipping_price,
      })
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

   pub async fn create_order_from_cart(&self, user_id: Uuid) -> Result<OrderModel, AppError> {
      // Create transaction
      let mut tx = self.pool.begin().await?;
      // Will be rollback automatically when transaction goes out of scope

      // 1. check if cart is empty
      let cart_item_quantity: i64 =
         sqlx::query_scalar("SELECT COUNT(*) FROM cart_items WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&mut *tx)
            .await?;

      if cart_item_quantity == 0 {
         return Err(AppError::Conflict("the cart is empty".to_string()));
      }

      // 2. Validate stock: item quantity < item avaiable
      let insuf_item_ids: Vec<Uuid> = sqlx::query_scalar(
         r#"
            SELECT ci.id
            FROM cart_items ci
            JOIN books b ON b.id = ci.book_id
            WHERE ci.user_id = $1 AND ci.quantity > b.available
         "#,
      )
      .bind(user_id)
      .fetch_all(&mut *tx)
      .await?;

      if insuf_item_ids.len() > 0 {
         return Err(AppError::Conflict(
            "Insufficient stock for some items in cart".to_string(),
         ));
      }

      // 3. calculate total price
      let total_price: Decimal = sqlx::query_scalar(
         r#"
            SELECT SUM(ci.quantity * b.price)
            FROM cart_items ci
            JOIN books b ON b.id = ci.book_id
            WHERE ci.user_id = $1
         "#,
      )
      .bind(user_id)
      .fetch_one(&mut *tx)
      .await?;

      // 4. create orders
      let order = sqlx::query_as::<_, OrderModel>(
         r#"
            INSERT INTO orders
               (user_id, total_price)
            VALUES ($1, $2)
            RETURNING
               id, user_id, total_price, order_status, created_at, updated_at
         "#,
      )
      .bind(user_id)
      .bind(total_price)
      .fetch_one(&mut *tx)
      .await?;

      // 5. copy cart items to order items
      sqlx::query(
         r#"
            INSERT INTO order_items
               (order_id, book_id, quantity, price_at_purchase)
            SELECT $1, ci.book_id, ci.quantity, b.price
            FROM cart_items ci
            JOIN books b ON b.id = ci.book_id
            WHERE ci.user_id = $2
         "#,
      )
      .bind(order.id)
      .bind(user_id)
      .execute(&mut *tx)
      .await?;

      // 6. clear cart items
      sqlx::query("DELETE FROM cart_items WHERE user_id = $1")
         .bind(user_id)
         .execute(&mut *tx)
         .await?;

      tx.commit().await?;

      Ok(order)
   }
}
