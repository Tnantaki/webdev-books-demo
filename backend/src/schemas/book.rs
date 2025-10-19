use std::str::FromStr;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::{
   models::books::{BookModel, SortBy, SortOrder},
   schemas::image::get_img_path_by_id,
};

#[derive(Serialize)]
pub struct Book {
   pub id: Uuid,
   pub title: String,
   pub genre: String,
   pub description: String,
   pub price_in_pound: Decimal,
   pub available: i32,
   pub img_path: String,
   pub average_rating: f64,
   pub total_ratings: i32,
   pub created_at: DateTime<Utc>,
   pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate)]
pub struct AddBook {
   #[validate(length(min = 4, max = 255))]
   pub title: String,
   #[validate(length(min = 1, max = 255))]
   pub genre: String,
   pub description: String,
   #[validate(custom(function = "validate_price"))]
   pub price_in_pound: Decimal,
   #[validate(range(min = 0, max = 999_999_999))]
   pub available: Option<i32>,
   pub image_id: Uuid,
}

#[derive(Deserialize, Validate)]
pub struct EditBook {
   #[validate(length(min = 4, max = 255))]
   pub title: Option<String>,
   #[validate(length(min = 1, max = 255))]
   pub genre: Option<String>,
   pub description: Option<String>,
   #[validate(custom(function = "validate_price"))]
   pub price_in_pound: Option<Decimal>,
   #[validate(range(min = 0, max = 999_999_999))]
   pub available: Option<i32>,
   pub image_id: Option<Uuid>,
}

impl From<BookModel> for Book {
   fn from(book: BookModel) -> Self {
      Self {
         id: book.id,
         title: book.title,
         genre: book.genre,
         description: book.description,
         price_in_pound: book.price_in_pound,
         available: book.available,
         img_path: get_img_path_by_id(book.image_id),
         average_rating: book.average_rating,
         total_ratings: book.total_ratings,
         created_at: book.created_at,
         updated_at: book.updated_at,
      }
   }
}

fn validate_price(price: &Decimal) -> Result<(), ValidationError> {
   if price.is_sign_negative() {
      return Err(
         ValidationError::new("range").with_message("price value can't be negative".into()),
      );
   }

   let max = Decimal::from_str("999999.99").unwrap();
   if *price > max {
      return Err(
         ValidationError::new("range").with_message("price value max at 999,999.99".into()),
      );
   }

   if price.scale() > 2 {
      return Err(
         ValidationError::new("decimal").with_message("price value max at 2 decimal points".into()),
      );
   }

   Ok(())
}

// fn validate_decimal_precision(value: f64) -> Result<(), ValidationError> {
//    let price_str = format!("{:.10}", value);
//    if let Some(dot_pos) = price_str.find('.') {
//       let decimal_part = &price_str[dot_pos + 1..];
//       let trimmed = decimal_part.trim_end_matches('0');
//       if trimmed.len() > 2 {
//          return Err(ValidationError::new("maxs 2 decimals"));
//       }
//    }
//    Ok(())
// }

// Pagination query parameters
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
   #[serde(default = "default_page")]
   pub page: i64,
   #[serde(default = "default_per_page")]
   pub per_page: i64,
   #[serde(default)]
   pub sort_by: SortBy,
   #[serde(default)]
   pub order: SortOrder,
   #[serde(default)]
   pub genre: Option<String>,
}

fn default_page() -> i64 {
   1
}
fn default_per_page() -> i64 {
   12
}

// Pagination response
#[derive(Debug, Serialize)]
pub struct PaginationResponse<T> {
   pub data: Vec<T>,
   pub pagination: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
   pub current_page: i64,
   pub per_page: i64,
   pub total_items: i64,
   pub total_pages: i64,
   pub has_next: bool,
   pub has_previous: bool,
}
