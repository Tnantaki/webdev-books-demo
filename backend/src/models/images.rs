use axum::body::Bytes;
use uuid::Uuid;

use crate::schemas::image::AddImage;

#[derive(Clone)]
pub struct ImageModel {
   pub id: Uuid,
   pub filename: String,
   pub content_type: String,
   pub data: Bytes,
}

impl ImageModel {
   pub fn add(image: AddImage) -> Self {
      Self {
         id: Uuid::now_v7(),
         filename: image.filename,
         content_type: image.content_type,
         data: image.data,
      }
   }
}
