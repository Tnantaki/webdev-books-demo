use crate::models::users::{Role, UserModel};
use chrono::TimeDelta;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct Token {
   pub user_id: Uuid,
   pub role: Role,
   pub exp: usize,
}

// Define payload of JWT (Information about the token)
// must Serailize and Deserialize for encode and decode
#[derive(Serialize, Deserialize)]
pub struct Claims {
   sub: Uuid,  // identity of user (name or id)
   role: Role, // access permission (admin or normal user)
   exp: usize, // expiration time of token
   iat: usize, // issued at: time when token was issue
}

#[derive(Clone)]
pub struct JwtTokenService {
   encoding_key: EncodingKey,
   decoding_key: DecodingKey,
}

#[derive(Error, Debug)]
pub enum JwtTokenError {
   #[error("{0}")]
   TokenGenerationFailed(String),

   #[error("invalid token {0}")]
   InvalidToken(String),
}

impl JwtTokenService {
   pub fn new(secret: &str) -> Self {
      Self {
         encoding_key: EncodingKey::from_secret(secret.as_bytes()),
         decoding_key: DecodingKey::from_secret(secret.as_bytes()),
      }
   }
}

impl JwtTokenService {
   pub fn generate(&self, user: &UserModel, duration: TimeDelta) -> Result<String, JwtTokenError> {
      let now = chrono::Utc::now();
      let expiration = chrono::Utc::now()
         .checked_add_signed(duration)
         .expect("Duration on generate token time is out of range")
         .timestamp() as usize;

      let claims = Claims {
         sub: user.id,
         role: user.role.clone(),
         exp: expiration,
         iat: now.timestamp() as usize,
      };

      let token = encode(&Header::default(), &claims, &self.encoding_key)
         .map_err(|e| JwtTokenError::TokenGenerationFailed(e.to_string()))?;

      Ok(token)
   }

   pub fn verify(&self, token: &str) -> Result<Token, JwtTokenError> {
      let token = decode::<Claims>(token, &self.decoding_key, &Validation::default())
         .map_err(|e| JwtTokenError::InvalidToken(e.to_string()))?;

      Ok(Token {
         user_id: token.claims.sub,
         role: token.claims.role,
         exp: token.claims.exp,
      })
   }

   pub fn refresh(
      &self,
      refresh_token: &str,
      duration: TimeDelta,
   ) -> Result<String, JwtTokenError> {
      let auth_token = self.verify(refresh_token)?;

      let now = chrono::Utc::now();
      let access_expiration = chrono::Utc::now()
         .checked_add_signed(duration)
         .expect("Duration on generate token time is out of range")
         .timestamp() as usize;

      let claims = Claims {
         sub: auth_token.user_id,
         role: auth_token.role,
         exp: access_expiration,
         iat: now.timestamp() as usize,
      };

      let access_token = encode(&Header::default(), &claims, &self.encoding_key)
         .map_err(|e| JwtTokenError::TokenGenerationFailed(e.to_string()))?;

      Ok(access_token)
   }
}
