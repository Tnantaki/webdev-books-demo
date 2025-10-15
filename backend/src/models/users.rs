use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "user_role")]
#[sqlx(rename_all = "lowercase")] // Converts User -> user, Admin -> admin
pub enum Role {
   User,
   Admin,
}

#[derive(Clone, FromRow)]
pub struct UserModel {
   pub id: Uuid,
   pub email: String,
   pub password_hash: String,
   pub role: Role,
   pub created_at: DateTime<Utc>,
   pub updated_at: DateTime<Utc>,
}
