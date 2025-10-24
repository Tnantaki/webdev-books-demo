pub mod books;
pub mod cart_items;
pub mod images;
pub mod orders;
pub mod ratings;
pub mod users;

use crate::{
   ServerError,
   repos::postgres::{
      books::BookRepo, cart_items::CartItemRepo, images::ImageRepo, orders::OrderRepo,
      ratings::RatingRepo, users::UserRepo,
   },
};
use sqlx::{Pool, Postgres};
use std::time::Duration;

#[derive(Clone)]
pub struct PostgresRepos {
   pub book_repo: BookRepo,
   pub user_repo: UserRepo,
   pub image_repo: ImageRepo,
   pub cart_item_repo: CartItemRepo,
   pub order_repo: OrderRepo,
   pub rating_repo: RatingRepo,
}

impl PostgresRepos {
   pub fn new(pool: Pool<Postgres>) -> Self {
      Self {
         book_repo: BookRepo::new(pool.clone()),
         user_repo: UserRepo::new(pool.clone()),
         image_repo: ImageRepo::new(pool.clone()),
         cart_item_repo: CartItemRepo::new(pool.clone()),
         order_repo: OrderRepo::new(pool.clone()),
         rating_repo: RatingRepo::new(pool),
      }
   }
}

pub async fn connect(db_url: &str) -> Result<Pool<Postgres>, ServerError> {
   let pool = sqlx::postgres::PgPoolOptions::new()
      .acquire_timeout(Duration::from_secs(3))
      .connect(db_url)
      .await
      .map_err(|e| ServerError::DatabaseError(e.to_string()))?;

   Ok(pool)
}
