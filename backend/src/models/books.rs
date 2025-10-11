use crate::schemas::book::{AddBook, EditBook};
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
   pub img_path: String,
   pub created_at: DateTime<Utc>,
   pub updated_at: DateTime<Utc>,
}

impl BookModel {
   pub fn add(new_book: AddBook) -> Self {
      let now: DateTime<Utc> = Utc::now();

      Self {
         id: Uuid::now_v7(),
         title: new_book.title,
         genre: new_book.genre,
         description: new_book.description,
         price_in_pound: new_book.price_in_pound,
         available: new_book.available.unwrap_or(0),
         img_path: new_book.img_path,
         created_at: now,
         updated_at: now,
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
      if let Some(img_path) = edit_book.img_path {
         self.img_path = img_path;
      }
      self.updated_at = Utc::now();
   }
}
