use crate::schemas::book::{AddBook, EditBook};
use rust_decimal::Decimal;
use uuid::Uuid;

// Database might not support non-negative values, So use integer
#[derive(Clone)]
pub struct BookModel {
   pub id: Uuid,
   pub title: String,
   pub genre: String,
   pub description: String,
   pub price_in_pound: Decimal,
   pub available: i32,
   // pub img_path: String
}

impl BookModel {
   pub fn add(new_book: AddBook) -> Self {
      Self {
         id: Uuid::now_v7(),
         title: new_book.title,
         genre: new_book.genre,
         description: new_book.description,
         price_in_pound: new_book.price_in_pound,
         available: new_book.available.unwrap_or(0),
      }
   }

   pub fn edit(&mut self, edit_book: EditBook) {
      if let Some(title) = edit_book.title {
         self.title = title;
      }
      if let Some(genre) = edit_book.genre {
         self.genre = genre;
      }
      if let Some(description) = edit_book.description {
         self.description = description;
      }
      if let Some(price_in_pound) = edit_book.price_in_pound {
         self.price_in_pound = price_in_pound;
      }
      if let Some(available) = edit_book.available {
         self.available = available;
      }
   }
}

// fn validate_price_decimal_float(value: f64) -> Result<(), ValidationError> {
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
