use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use serde_json::json;
use validator::Validate;

use crate::{
   routes::app_error::AppError, schemas::user::RegisterUser,
   services::password_hashing::PasswordService, startup::app_state::AppState,
};

pub fn router(state: AppState) -> Router {
   Router::new().route("/register", post(register_user)).with_state(state)
}

async fn register_user(
   State(state): State<AppState>,
   Json(payload): Json<RegisterUser>,
) -> Result<impl IntoResponse, AppError> {
   // 1. Validate input
   payload.validate()?;

   // 2. Check unique email
   if state.user_repo.view_user_by_email(&payload.email).is_ok() {
      return Err(AppError::Conflict("Email already exists.".to_string()));
   }

   // 3. Hashing password
   let password_hash = PasswordService::hash(&payload.password)?;

   // 4. Save to database
   state.user_repo.add_user(payload.email, password_hash)?;

   // 5. Response
   Ok((
      StatusCode::CREATED,
      Json(json!({"message": "register user successfully."})),
   ))
}
