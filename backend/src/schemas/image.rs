use crate::{models::images::ImageMetadata, routes::app_error::AppError};
use chrono::{DateTime, Utc};
use core::fmt;
use serde::Serialize;
use uuid::Uuid;

// Supported image extensions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImgType {
   Jpg,
   Jpeg,
   Png,
   Gif,
   Webp,
   Bmp,
   Svg,
}

pub struct AddImage {
   pub filename: String,
   pub content_type: String,
   pub data: Vec<u8>,
}

#[derive(Serialize)]
pub struct ImageResponse {
   pub id: Uuid,
   pub filename: String, // To tell the client which file be change to id
   pub img_path: String,
   pub created_at: DateTime<Utc>,
}

impl AddImage {
   pub fn new(filename: String, img_type: ImgType, data: Vec<u8>) -> Self {
      Self {
         filename,
         content_type: img_type.content_type().to_string(),
         data,
      }
   }
}

pub fn get_img_path_by_id(image_id: Uuid) -> String {
   format!("/images/{}", image_id)
}

impl From<ImageMetadata> for ImageResponse {
   fn from(img: ImageMetadata) -> Self {
      // let extension = ImageExtension::from_content_type(&img.content_type);
      // let img_path = format!("/images/{}.{}", img.id, extension);

      Self {
         id: img.id,
         filename: img.filename,
         img_path: get_img_path_by_id(img.id),
         created_at: img.created_at,
      }
   }
}

impl fmt::Display for ImgType {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.extension())
   }
}

impl ImgType {
   /// Get extension file type
   pub fn extension(&self) -> &'static str {
      match self {
         Self::Jpg => "jpg",
         Self::Jpeg => "jpeg",
         Self::Png => "png",
         Self::Gif => "gif",
         Self::Webp => "webp",
         Self::Bmp => "bmp",
         Self::Svg => "svg",
      }
   }

   /// Get MIME type / Content-Type
   pub fn content_type(&self) -> &'static str {
      match self {
         Self::Jpg | Self::Jpeg => "image/jpeg",
         Self::Png => "image/png",
         Self::Gif => "image/gif",
         Self::Webp => "image/webp",
         Self::Bmp => "image/bmp",
         Self::Svg => "image/svg+xml",
      }
   }

   /// Parse from content type
   pub fn from_content_type(content_type: &str) -> Result<Self, AppError> {
      let img_type = match content_type.to_lowercase().as_str() {
         "image/jpeg" => Self::Jpeg,
         "image/png" => Self::Png,
         "image/gif" => Self::Gif,
         "image/webp" => Self::Webp,
         "image/bmp" => Self::Bmp,
         "image/svg+xml" => Self::Svg,
         _ => {
            return Err(AppError::UploadFile(format!(
               "Unsupported content type {content_type}"
            )));
         }
      };
      Ok(img_type)
   }

   /// Parse extension from string
   pub fn from_extension(ext: &str) -> Result<Self, AppError> {
      let img_type = match ext.to_lowercase().as_str() {
         "jpg" => Self::Jpg,
         "jpeg" => Self::Jpeg,
         "png" => Self::Png,
         "gif" => Self::Gif,
         "webp" => Self::Webp,
         "bmp" => Self::Bmp,
         "svg" => Self::Svg,
         _ => return Err(AppError::UploadFile(format!("Unsupported extension {ext}"))),
      };
      Ok(img_type)
   }
}
