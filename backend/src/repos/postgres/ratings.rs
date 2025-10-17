use sqlx::PgPool;
use uuid::Uuid;

use crate::{
   models::ratings::{BookRatingModel, RatingModel},
   routes::app_error::AppError,
   schemas::rating::GiveRate,
};

#[derive(Clone)]
pub struct RatingRepo {
   pool: PgPool,
}

impl RatingRepo {
   pub fn new(pool: PgPool) -> Self {
      Self { pool }
   }

   pub async fn give_rate(
      &self,
      user_id: Uuid,
      book_id: Uuid,
      give_rate: GiveRate,
   ) -> Result<RatingModel, AppError> {
      // Purchased user only allow to rating the book
      let has_purchased = sqlx::query_scalar::<_, bool>(
         "SELECT EXISTS(
               SELECT 1 FROM order_items oi
               JOIN orders o ON o.id = oi.order_id
               WHERE user_id = $1 AND order_status = 'paid' AND book_id = $2
            )
         ",
      )
      .bind(user_id)
      .bind(book_id)
      .fetch_one(&self.pool)
      .await?;

      if !has_purchased {
         return Err(AppError::Conflict(
            "must purchase this book before rating it.".to_string(),
         ));
      }

      // rating the book
      let rate = sqlx::query_as::<_, RatingModel>(
         r#"
            INSERT INTO ratings (user_id, book_id, rating, review)
            VALUES ($1, $2, $3, $4)
            RETURNING
               id, book_id, user_id, rating, review, created_at
         "#,
      )
      .bind(user_id)
      .bind(book_id)
      .bind(give_rate.rating)
      .bind(give_rate.review.as_ref())
      .fetch_one(&self.pool)
      .await?;

      Ok(rate)
   }

   // Add or Update rating - Automatically inserts or updates based on existence
   pub async fn upsert_rating(
      &self,
      user_id: Uuid,
      book_id: Uuid,
      give_rate: GiveRate,
   ) -> Result<RatingModel, AppError> {
      // Check if user has purchased the book
      let has_purchased = sqlx::query_scalar::<_, bool>(
         "SELECT EXISTS(
               SELECT 1 FROM order_items oi
               JOIN orders o ON o.id = oi.order_id
               WHERE user_id = $1 AND order_status = 'paid' AND book_id = $2
            )
         ",
      )
      .bind(user_id)
      .bind(book_id)
      .fetch_one(&self.pool)
      .await?;

      if !has_purchased {
         return Err(AppError::Conflict(
            "must purchase this book before rating it.".to_string(),
         ));
      }

      // Use UPSERT (INSERT ... ON CONFLICT ... DO UPDATE)
      // This will insert if not exists, or update if exists
      let upserted_rating = sqlx::query_as::<_, RatingModel>(
         r#"
            INSERT INTO ratings (user_id, book_id, rating, review)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_id, book_id)
            DO UPDATE SET
               rating = EXCLUDED.rating,
               review = EXCLUDED.review
            RETURNING
               id, book_id, user_id, rating, review, created_at
         "#,
      )
      .bind(user_id)
      .bind(book_id)
      .bind(give_rate.rating)
      .bind(give_rate.review.as_ref())
      .fetch_one(&self.pool)
      .await?;

      Ok(upserted_rating)
   }

   pub async fn get_book_rating(&self, book_id: Uuid) -> Result<Vec<BookRatingModel>, AppError> {
      let book_ratings = sqlx::query_as::<_, BookRatingModel>(
         r#"
            SELECT  id, user_id, rating, review, created_at
            FROM ratings
            WHERE book_id = $1
         "#,
      )
      .bind(book_id)
      .fetch_all(&self.pool)
      .await?;

      if book_ratings.is_empty() {
         return Ok(vec![]);
      }

      Ok(book_ratings)
   }
}
