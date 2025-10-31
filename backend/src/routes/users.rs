use axum::{
   Json, Router,
   extract::{Path, State},
   http::StatusCode,
   middleware,
   routing::{delete, get},
};
use uuid::Uuid;

use crate::{
   routes::{JsonResult, app_error::AppError, middleware::auth_cookie_admin},
   schemas::user::User,
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
