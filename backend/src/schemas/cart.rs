use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::schemas::book::Book;

// Request Schema
#[derive(Deserialize, Validate)]
pub struct AddCartItem {
   pub book_id: Uuid,
   #[validate(range(min = 0, max = 999))]
   pub quantity: i32,
}

#[derive(Deserialize, Validate)]
pub struct EditCartItem {
   #[validate(range(min = 0, max = 999))]
   pub quantity: i32,
}

// Response Schema
#[derive(Serialize)]
pub struct CartItem {
   pub id: Uuid,
   pub book: Book,
   pub quantity: i32,
   pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct Cart {
   pub items: Vec<CartItem>,
   pub total_price: Decimal,
   pub shipping_price: Decimal,
}
