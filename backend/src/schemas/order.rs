use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

// Response Schema
#[derive(Serialize, Clone)]
pub struct BookAtPurchase {
   pub book_id: Uuid,
   pub title: String,
   pub genre: String,
   pub img_path: String,
   pub quantity: i32,
   pub price_at_purchase: Decimal,
}

#[derive(Serialize)]
pub struct OrderDetail {
   pub id: Uuid,
   pub total_price: Decimal,
   pub order_status: String,
   pub created_at: DateTime<Utc>,
   pub updated_at: DateTime<Utc>,
   pub items: Vec<BookAtPurchase>,
}

// For admin
#[derive(Serialize)]
pub struct Order {
   pub id: Uuid,
   pub user_id: Uuid,
   pub total_price: Decimal,
   pub order_status: String,
   pub created_at: DateTime<Utc>,
   pub updated_at: DateTime<Utc>,
}
