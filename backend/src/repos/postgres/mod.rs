pub mod books;

use crate::{ServerError, repos::postgres::books::BookRepo};
use sqlx::{Pool, Postgres};
use std::time::Duration;

#[derive(Clone)]
pub struct PostgresRepos {
   pub book_repo: BookRepo,
}

impl PostgresRepos {
   pub fn new(pool: Pool<Postgres>) -> Self {
      Self {
         book_repo: BookRepo::new(pool),
      }
   }
}

pub async fn connect(db_url: &str) -> Result<Pool<Postgres>, ServerError<'static>> {
   let pool = sqlx::postgres::PgPoolOptions::new()
      .acquire_timeout(Duration::from_secs(3))
      .connect(db_url)
      .await
      .map_err(|e| ServerError::DatabaseError(e.to_string()))?;

   Ok(pool)
}
