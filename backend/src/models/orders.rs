use std::fmt;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::schemas::order::Order;

// Map to 'order' table in database
#[derive(Clone, FromRow, Serialize)]
pub struct OrderModel {
   pub id: Uuid,
   pub user_id: Uuid,
   pub total_price: Decimal,
   pub order_status: String,
   pub created_at: DateTime<Utc>,
   pub updated_at: DateTime<Utc>,
}

// For query data for user
#[derive(Clone, FromRow)]
pub struct OrderDetailModel {
   pub id: Uuid, // order id
   pub total_price: Decimal,
   pub order_status: String,
   pub created_at: DateTime<Utc>,
   pub updated_at: DateTime<Utc>,
   pub book_id: Uuid,
   pub title: String,
   pub genre: String,
   pub image_id: Uuid,
   pub quantity: i32,              // at order item
   pub price_at_purchase: Decimal, // at order item
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
   Pending,
   Paid,
   Completed,
}

// Map to Order Status model in database
impl fmt::Display for OrderStatus {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
         OrderStatus::Pending => write!(f, "pending"),
         OrderStatus::Paid => write!(f, "paid"),
         OrderStatus::Completed => write!(f, "completed"),
      }
   }
}

impl From<OrderModel> for Order {
   fn from(value: OrderModel) -> Self {
      Self {
         id: value.id,
         user_id: value.user_id,
         total_price: value.total_price,
         order_status: value.order_status,
         created_at: value.created_at,
         updated_at: value.updated_at,
      }
   }
}
