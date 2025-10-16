use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

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