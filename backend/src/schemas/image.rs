use axum::body::Bytes;
use serde::Serialize;
use uuid::Uuid;

pub struct AddImage {
   pub filename: String,
   pub content_type: String,
   pub data: Bytes,
}

#[derive(Serialize)]
pub struct ImageResponse {
   pub id: Uuid,
   pub filename: String,
   pub url: String,
}

