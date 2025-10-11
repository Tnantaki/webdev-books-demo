use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Serialize)]
pub struct User {
   pub id: Uuid,
   pub email: String,
}

#[derive(Deserialize, Validate)]
#[validate(schema(function = "validate_passwords_match"))]
pub struct RegisterUser {
   #[validate(email)]
   pub email: String,
   #[validate(length(min = 1, message = "Password must be at least 8 characters long"))]
   #[validate(custom(function = "validate_password_strength"))]
   pub password: String,
   #[validate(length(min = 1, max = 255))]
   pub confirm_password: String,
}

// Validate that password and confirm_password match
fn validate_passwords_match(input: &RegisterUser) -> Result<(), ValidationError> {
   if input.password != input.confirm_password {
      return Err(
         ValidationError::new("passwords_mismatch").with_message("Passwords do not match".into()),
      );
   }
   Ok(())
}

// Custom validation for password strength
fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
   let has_uppercase = password.chars().any(|c| c.is_uppercase());
   let has_lowercase = password.chars().any(|c| c.is_lowercase());
   let has_digit = password.chars().any(|c| c.is_numeric());

   if !has_uppercase || !has_lowercase || !has_digit {
      return Err(ValidationError::new("password_weak")
           .with_message(std::borrow::Cow::from(
               "Password must contain at least one uppercase letter, one lowercase letter, and one digit"
           )));
   }
   Ok(())
}
