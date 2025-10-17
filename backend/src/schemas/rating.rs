use serde::Deserialize;
use validator::Validate;


#[derive(Deserialize, Validate)]
pub struct GiveRate {
   #[validate(range(min = 1, max = 5))]
   pub rating: i16,
   #[validate(length(min = 0, max = 1000))]
   pub review: Option<String>,
}