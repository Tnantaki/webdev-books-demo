use uuid::Uuid;

#[derive(Clone)]
pub struct UserModel {
   pub id: Uuid,
   pub email: String,
   pub password_hash: String,
}
