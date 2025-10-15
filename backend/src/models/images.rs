use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, FromRow)]
pub struct ImageModel {
   pub id: Uuid,
   pub filename: String,
   pub content_type: String,
   pub data: Vec<u8>,
   pub file_size: i32,
   pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct ImageMetadata {
   pub id: Uuid,
   pub filename: String,
   pub content_type: String,
   pub file_size: i32,
   pub created_at: DateTime<Utc>,
}
