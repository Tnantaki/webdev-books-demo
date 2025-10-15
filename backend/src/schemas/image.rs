use crate::{models::images::ImageMetadata, routes::app_error::AppError};
use chrono::{DateTime, Utc};
use core::fmt;
use serde::Serialize;
use uuid::Uuid;

// Supported image extensions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageExtension {
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
   pub fn new(filename: String, content_type: String, data: Vec<u8>) -> Self {
      Self {
         filename,
         content_type,
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

impl fmt::Display for ImageExtension {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.as_str())
   }
}

impl ImageExtension {
   pub fn from_content_type(ext: &str) -> String {
      let extension = match ext.to_lowercase().as_str() {
         "image/jpeg" => Self::Jpeg,
         "image/png" => Self::Png,
         "image/gif" => Self::Gif,
         "image/webp" => Self::Webp,
         "image/bmp" => Self::Bmp,
         "image/svg+xml" => Self::Svg,
         _ => Self::Jpg,
      };
      extension.as_str().to_string()
   }

   /// Convert to string representation
   pub fn as_str(&self) -> &'static str {
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
   pub fn is_allow_content_type(content_type: &str) -> Result<Self, AppError> {
      match content_type.to_lowercase().as_str() {
         "image/jpeg" => Ok(Self::Jpeg),
         "image/png" => Ok(Self::Png),
         "image/gif" => Ok(Self::Gif),
         "image/webp" => Ok(Self::Webp),
         "image/bmp" => Ok(Self::Bmp),
         "image/svg+xml" => Ok(Self::Svg),
         _ => Err(AppError::UploadFile(format!(
            "Unsupported content type {content_type}"
         ))),
      }
   }
}
