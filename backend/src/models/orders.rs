use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, FromRow, Serialize)]
pub struct OrderModel {
   pub id: Uuid,
   pub user_id: Uuid,
   pub total_price: Decimal,
   pub order_status: String,
   pub created_at: DateTime<Utc>,
   pub updated_at: DateTime<Utc>,
}

#[derive(Clone, FromRow, Serialize)]
pub struct OrderItemDetail {
   pub id: Uuid,
   pub book_id: Uuid,
   pub genre: String,
   pub description: String,
   pub image_id: Uuid,
   pub quantity: i32,
   pub price_at_purchase: Decimal,
}
