use argon2::{
   Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
   password_hash::{SaltString, rand_core::OsRng},
};
use thiserror::Error;

pub struct PasswordService;

#[derive(Error, Debug)]
pub enum PasswordHashError {
   #[error("{0}")]
   HashFail(String),

   #[error("{0}")]
   VerifyFail(String),
}

impl PasswordService {
   pub fn hash(password: &str) -> Result<String, PasswordHashError> {
      let salt = SaltString::generate(&mut OsRng);

      // Hash password
      Argon2::default()
         .hash_password(password.as_bytes(), &salt)
         .map(|hash| Ok(hash.to_string()))
         .map_err(|error| PasswordHashError::HashFail(error.to_string()))?
   }

   pub fn verify(password: &str, password_hash: &str) -> Result<bool, PasswordHashError> {
      let parsed_hash = PasswordHash::new(&password_hash)
         .map_err(|error| PasswordHashError::HashFail(error.to_string()))?;

      // Verify password
      Argon2::default()
         .verify_password(password.as_bytes(), &parsed_hash)
         .map_err(|error| PasswordHashError::VerifyFail(error.to_string()))?;

      Ok(true)
   }
}
