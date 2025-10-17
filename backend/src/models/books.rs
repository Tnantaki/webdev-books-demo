use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::prelude::FromRow;
use uuid::Uuid;

// Database might not support non-negative values, So use integer
#[derive(Clone, FromRow)]
pub struct BookModel {
   pub id: Uuid,
   pub title: String,
   pub genre: String,
   pub description: String,
   pub price_in_pound: Decimal,
   pub available: i32,
   pub image_id: Uuid,
   pub average_rating: f64,
   pub total_ratings: i32,
   pub created_at: DateTime<Utc>,
   pub updated_at: DateTime<Utc>,
}
