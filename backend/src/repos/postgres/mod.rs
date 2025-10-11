use crate::ServerError;
use sqlx::{Pool, Postgres};
use std::time::Duration;

pub async fn connect(db_url: &str) -> Result<Pool<Postgres>, ServerError<'static>> {
   let pool = sqlx::postgres::PgPoolOptions::new()
      .acquire_timeout(Duration::from_secs(3))
      .connect(db_url)
      .await
      .map_err(|e| ServerError::DatabaseError(e.to_string()))?;

   Ok(pool)
}
