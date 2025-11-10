use std::fmt;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::routes::app_error::AppError;

// Database might not support non-negative values, So use integer
#[derive(Clone, FromRow)]
pub struct BookModel {
   pub id: Uuid,
   pub title: String,
   pub genre: String,
   pub description: String,
   pub price: Decimal,
   pub available: i32,
   pub image_id: Uuid,
   pub average_rating: f64,
   pub total_ratings: i32,
   pub created_at: DateTime<Utc>,
   pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortBy {
   #[default]
   Title,
   Genre,
   Price,
   Rating,
}

impl SortBy {
   pub fn new(sort: &str) -> Result<Self, AppError> {
      let sort_by = match sort {
         "tile" => SortBy::Title,
         "genre" => SortBy::Genre,
         "price" => SortBy::Price,
         "rating" => SortBy::Rating,
         _ => {
            return Err(AppError::BadRequest(
               "Invalid sorting book parameter".to_string(),
            ));
         }
      };
      Ok(sort_by)
   }
}

// Map to Book model in database
impl fmt::Display for SortBy {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
         SortBy::Title => write!(f, "title"),
         SortBy::Genre => write!(f, "genre"),
         SortBy::Price => write!(f, "price"),
         SortBy::Rating => write!(f, "average_rating"),
      }
   }
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
   #[default]
   Asc,
   Desc,
}

impl SortOrder {
   pub fn new(order: &str) -> Result<Self, AppError> {
      let sort_order = match order {
         "asc" => SortOrder::Asc,
         "desc" => SortOrder::Desc,
         _ => {
            return Err(AppError::BadRequest(
               "Invalid sorting order book parameter".to_string(),
            ));
         }
      };
      Ok(sort_order)
   }
}

impl fmt::Display for SortOrder {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
         SortOrder::Asc => write!(f, "ASC"),
         SortOrder::Desc => write!(f, "DESC"),
      }
   }
}

pub struct BookFilter {
   pub limit: Option<i64>,
   pub sort_by: SortBy,
   pub sort_order: SortOrder,
   pub genre: Option<String>,
   pub offset: Option<i64>,
}

impl BookFilter {
   pub fn new(
      limit: Option<i64>,
      sort_by: SortBy,
      sort_order: SortOrder,
      genre: Option<String>,
      offset: Option<i64>,
   ) -> Self {
      BookFilter {
         limit,
         sort_by,
         sort_order,
         genre,
         offset,
      }
   }
}
