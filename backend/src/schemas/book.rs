use std::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::models::books::BookModel;

#[derive(Serialize)]
pub struct Book {
   pub id: Uuid,
   pub title: String,
   pub genre: String,
   pub description: String,
   pub price_in_pound: Decimal,
   pub available: i32,
   pub img_path: String,
   // pub averate_rating: f32,
   // pub total_ratings: i32,
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
   #[validate(length(min = 0, max = 255))]
   pub img_path: String,
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
   #[validate(length(min = 0, max = 255))]
   pub img_path: Option<String>,
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
         img_path: book.img_path,
      }
   }
}

fn validate_price(price: &Decimal) -> Result<(), ValidationError> {
   if price.is_sign_negative() {
      return Err(ValidationError::new("price value can't be negative"));
   }

   let max = Decimal::from_str("999999.99").unwrap();
   if *price > max {
      return Err(ValidationError::new("price value max at 999,999.99"));
   }

   if price.scale() > 2 {
      return Err(ValidationError::new("price value max at 2 decimal points"));
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
