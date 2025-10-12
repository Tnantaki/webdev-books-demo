use axum::{
   Json, Router,
   extract::{Path, State},
   http::StatusCode,
   middleware,
   response::IntoResponse,
   routing::{delete, get, post},
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
   models::users::Role,
   routes::{JsonResult, app_error::AppError, middleware::auth_cookie_admin},
   schemas::user::{RegisterUser, User},
   services::password_hashing::PasswordService,
   startup::app_state::AppState,
};

pub fn router(state: &AppState) -> Router<AppState> {
   Router::new()
      .route("/", get(get_users))
      .route("/{id}", get(get_user_by_id))
      .route("/{id}", delete(delete_user))
      .route_layer(middleware::from_fn_with_state(
         state.jwt_service.clone(),
         auth_cookie_admin,
      ))
      .route("/register", post(register_user))
}

async fn register_user(
   State(state): State<AppState>,
   Json(payload): Json<RegisterUser>,
) -> Result<impl IntoResponse, AppError> {
   // 1. Validate input
   payload.validate()?;

   // 2. Check unique email
   if state.postgres.user_repo.get_user_by_email(&payload.email).await.is_ok() {
      return Err(AppError::Conflict("Email already exists.".to_string()));
   }

   // 3. Hashing password
   let password_hash = PasswordService::hash(&payload.password)?;

   // 4. Save to database
   // state.in_mem.user_repo.add_user(payload.email, password_hash, Role::User)?;
   state.postgres.user_repo.add_user(payload.email, password_hash, Role::User).await?;

   // 5. Response
   Ok((
      StatusCode::CREATED,
      Json(json!({"message": "register user successfully."})),
   ))
}

async fn get_users(State(state): State<AppState>) -> JsonResult<Vec<User>> {
   let users = state.postgres.user_repo.get_all_user().await?;

   Ok((StatusCode::OK, Json(users)))
}

async fn get_user_by_id(State(state): State<AppState>, Path(id): Path<Uuid>) -> JsonResult<User> {
   let user = state.postgres.user_repo.get_user_by_id(id).await?;

   Ok((StatusCode::OK, Json(user)))
}

// TODO: input will be old password and new password
// async fn edit_password(State(state): State<AppState>, Path(id): Path<Uuid>) -> JsonResult<User> {

async fn delete_user(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
   state.postgres.user_repo.delete_user(id).await?;

   Ok(StatusCode::NO_CONTENT)
}
