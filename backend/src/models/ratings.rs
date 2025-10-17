use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, FromRow, Serialize)]
pub struct RatingModel {
   pub id: Uuid,
   pub book_id: Uuid,
   pub user_id: Uuid,
   pub rating: i16,
   pub review: Option<String>,
   pub created_at: DateTime<Utc>,
}

#[derive(Clone, FromRow, Serialize)]
pub struct BookRatingModel {
   pub id: Uuid,
   pub user_id: Uuid,
   pub rating: i16,
   pub review: Option<String>,
   pub created_at: DateTime<Utc>,
}