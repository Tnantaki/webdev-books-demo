use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, FromRow, Serialize)]
pub struct CartItemModel {
   pub id: Uuid,
   pub user_id: Uuid,
   pub book_id: Uuid,
   pub quantity: i32,
   pub created_at: DateTime<Utc>,
   pub updated_at: DateTime<Utc>,
}
