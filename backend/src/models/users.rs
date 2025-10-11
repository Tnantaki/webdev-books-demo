use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
   User,
   Admin,
}

#[derive(Clone)]
pub struct UserModel {
   pub id: Uuid,
   pub email: String,
   pub password_hash: String,
   pub role: Role,
}

impl UserModel {
   pub fn add_user(email: String, password_hash: String) -> Self {
      Self {
         id: Uuid::now_v7(),
         email,
         password_hash,
         role: Role::User
      }
   }
   
   pub fn add_admin(email: String, password_hash: String) -> Self {
      Self {
         id: Uuid::now_v7(),
         email,
         password_hash,
         role: Role::Admin
      }
   }
}
