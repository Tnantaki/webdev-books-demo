use uuid::Uuid;

#[derive(Clone)]
pub struct UserModel {
   pub id: Uuid,
   pub email: String,
   pub password_hash: String,
}

impl UserModel {
   pub fn add(email: String, password_hash: String) -> Self {
      Self {
         id: Uuid::now_v7(),
         email,
         password_hash,
      }
   }
}
