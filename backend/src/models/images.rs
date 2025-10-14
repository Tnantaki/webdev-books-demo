use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::schemas::image::AddImage;

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

// for in-memory
impl ImageModel {
   pub fn add(image: AddImage) -> Self {
      let file_size = image.data.len() as i32;

      Self {
         id: Uuid::now_v7(),
         filename: image.filename,
         content_type: image.content_type,
         data: image.data,
         file_size,
         created_at: Utc::now(),
      }
   }
}
