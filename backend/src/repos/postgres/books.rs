use crate::{
   models::books::BookModel,
   routes::app_error::AppError,
   schemas::book::{AddBook, Book, EditBook},
};
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
               id, title, genre, description, price_in_pound, available, img_path, created_at, updated_at
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
               id, title, genre, description, price_in_pound, available, img_path, created_at, updated_at
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
      let book = BookModel::add(new_book);

      let book_model = sqlx::query_as::<_, BookModel>(
         r#"
            INSERT INTO books
               (id, title, genre, description, price_in_pound, available, img_path, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
               id, title, genre, description, price_in_pound, available, img_path, created_at, updated_at
         "#,
      )
      .bind(&book.title)
      .bind(&book.genre)
      .bind(&book.description)
      .bind(&book.price_in_pound)
      .bind(&book.available)
      .bind(&book.img_path)
      .bind(&book.created_at)
      .bind(&book.updated_at)
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
               price_in_pound = COALESCE($4, price_in_pound),
               available = COALESCE($5, available),
               img_path = COALESCE($6, img_path)
            WHERE id = $7
            RETURNING
               id, title, genre, description, price_in_pound, available, img_path, created_at, updated_at
         "#,
      )
      .bind(edit_book.title.as_ref())
      .bind(edit_book.genre.as_ref())
      .bind(edit_book.description.as_ref())
      .bind(edit_book.price_in_pound.as_ref())
      .bind(edit_book.available.as_ref())
      .bind(edit_book.img_path.as_ref())
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
}
