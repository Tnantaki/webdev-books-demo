use console::style;
use sqlx::{Pool, Postgres};

use crate::ServerError;

pub async fn empty_all_tables(pool: Pool<Postgres>) -> Result<(), ServerError> {
   sqlx::query(
      r#"
	         TRUNCATE TABLE
					books, cart_items, images, orders, order_items, ratings, users
	         RESTART IDENTITY CASCADE;
         "#,
   )
   .execute(&pool)
   .await
   .map_err(|e| ServerError::CleanDataError(e.to_string()))?;
   
   println!("{}", style("\n✅ All tables truncated successfully!").green());
   Ok(())
}
