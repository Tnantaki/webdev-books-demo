use crate::{
   models::books::{BookFilter, BookModel},
   routes::app_error::AppError,
   schemas::book::{AddBook, Book, EditBook},
};
use rust_decimal::prelude::Zero;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct BookRepo {
   pool: PgPool,
}

impl BookRepo {
   pub fn new(pool: PgPool) -> Self {
      Self { pool }
   }

   pub async fn get_all_book(&self) -> Result<Vec<Book>, AppError> {
      let book_models = sqlx::query_as::<_, BookModel>(
         r#"
            SELECT
               id, title, genre, description, price, available, image_id,
               average_rating, total_ratings, created_at, updated_at
            FROM books
         "#,
      )
      .fetch_all(&self.pool)
      .await?;

      if book_models.is_empty() {
         return Ok(vec![]);
      }

      let books = book_models.into_iter().map(|book| Book::from(book)).collect();
      Ok(books)
   }

   pub async fn get_book_by_id(&self, id: Uuid) -> Result<Book, AppError> {
      let book_model = sqlx::query_as::<_, BookModel>(
         r#"
            SELECT
               id, title, genre, description, price, available, image_id,
               average_rating, total_ratings, created_at, updated_at
            FROM books WHERE id = $1
         "#,
      )
      .bind(id)
      .fetch_optional(&self.pool)
      .await?
      .ok_or(AppError::NotFound("invalid book id".to_string()))?;

      Ok(Book::from(book_model))
   }

   pub async fn add_book(&self, new_book: AddBook) -> Result<Book, AppError> {
      let book_model = sqlx::query_as::<_, BookModel>(
         r#"
            INSERT INTO books
               (title, genre, description, price, available, image_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
               id, title, genre, description, price, available, image_id,
               average_rating, total_ratings, created_at, updated_at
         "#,
      )
      .bind(&new_book.title)
      .bind(&new_book.genre)
      .bind(&new_book.description)
      .bind(&new_book.price)
      .bind(new_book.available.unwrap_or(0)) // not reference because i32 is copy type
      .bind(&new_book.image_id)
      .fetch_one(&self.pool)
      .await?;

      Ok(Book::from(book_model))
   }

   pub async fn edit_book(&self, id: Uuid, edit_book: EditBook) -> Result<Book, AppError> {
      let book_model = sqlx::query_as::<_, BookModel>(
         r#"
            UPDATE books SET
               title = COALESCE($1, title),
               genre = COALESCE($2, genre),
               description = COALESCE($3, description),
               price = COALESCE($4, price),
               available = COALESCE($5, available),
               image_id = COALESCE($6, image_id)
            WHERE id = $7
            RETURNING
               id, title, genre, description, price, available, image_id,
               average_rating, total_ratings, created_at, updated_at
         "#,
      )
      .bind(edit_book.title.as_ref())
      .bind(edit_book.genre.as_ref())
      .bind(edit_book.description.as_ref())
      .bind(edit_book.price.as_ref())
      .bind(edit_book.available.as_ref())
      .bind(edit_book.image_id.as_ref())
      .bind(id)
      .fetch_optional(&self.pool)
      .await?
      .ok_or(AppError::NotFound("invalid book id".to_string()))?;

      Ok(Book::from(book_model))
   }

   pub async fn delete_book(&self, id: Uuid) -> Result<(), AppError> {
      let row_affects = sqlx::query("DELETE FROM books WHERE id = $1")
         .bind(id)
         .execute(&self.pool)
         .await?
         .rows_affected();

      if row_affects == 0 {
         Err(AppError::NotFound("invalid book id".to_string()))
      } else {
         Ok(())
      }
   }

   pub async fn get_all_book_filter(
      &self,
      filter: BookFilter,
   ) -> Result<(i64, Vec<Book>), AppError> {
      let mut query = sqlx::QueryBuilder::new(
         r#"
            SELECT
               id, title, genre, description, price, available, image_id,
               average_rating, total_ratings, created_at, updated_at
            FROM books
      "#,
      );

      let mut query_count = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM books");

      // Filter Genre
      if let Some(genre) = filter.genre {
         query.push(" WHERE genre = ").push_bind(genre.to_string());
         query_count.push(" WHERE genre = ").push_bind(genre);
      }

      // Count total item by filter, The rest filter didn't effect number of item.
      let total_items: i64 = query_count.build_query_scalar().fetch_one(&self.pool).await?;

      if total_items.is_zero() {
         return Ok((0, vec![]));
      }

      // Filter Order and Sorting order
      query.push(format!(
         " ORDER BY {} {}",
         filter.sort_by, filter.sort_order
      ));

      // Filter Limit (limit item per page)
      if let Some(limit) = filter.limit {
         query.push(" LIMIT ").push_bind(limit);
      }

      // Filter Offset (offset page)
      if let Some(offset) = filter.offset {
         query.push(" OFFSET ").push_bind(offset);
      }

      let book_models = query.build_query_as::<BookModel>().fetch_all(&self.pool).await?;

      let books = book_models.into_iter().map(|book| Book::from(book)).collect();
      Ok((total_items, books))
   }
   
   pub async fn get_book_genre(&self) -> Result<Vec<String>, AppError> {
      let book_genre: Vec<String> = sqlx::query_scalar(
         r#"
	         SELECT DISTINCT genre
	         FROM books
	      "#,
      )
      .fetch_all(&self.pool)
      .await?;

      Ok(book_genre)
   }
}
