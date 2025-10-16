use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use chrono::{Duration, TimeDelta};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{
   Cookie, Cookies,
   cookie::{self, time},
};

use crate::{
   routes::app_error::AppError, services::password_hashing::PasswordService,
   startup::app_state::AppState,
};

const ACCESS_TOKEN_TIME: TimeDelta = Duration::hours(2);
const RREFRESH_TOKEN_TIME: TimeDelta = Duration::days(14);
const COOKIE_MAX_AGE: time::Duration = time::Duration::days(14);

#[derive(Deserialize)]
pub struct Login {
   email: String,
   password: String,
}

pub fn router() -> Router<AppState> {
   Router::new()
      .route("/login", post(login))
      .route("/logout", post(logout))
      .route("/refresh", post(refresh_token))
}

async fn login(
   State(state): State<AppState>,
   cookies: Cookies,
   Json(payload): Json<Login>,
) -> Result<impl IntoResponse, AppError> {
   // 1. Find email in repo
   let user = state
      .postgres
      .user_repo
      .get_user_by_email(&payload.email)
      .await
      .map_err(|_| AppError::Unauthorized("Invalid email".to_string()))?;

   // 2. Verrify user by email & password
   PasswordService::verify(&payload.password, &user.password_hash)?;

   // 3. Generate token
   let access_token = state.jwt_service.generate(&user, ACCESS_TOKEN_TIME)?;
   let refresh_token = state.jwt_service.generate(&user, RREFRESH_TOKEN_TIME)?;

   // 4. Build cookie by token
   let act_cookie = build_cookie("act", access_token, COOKIE_MAX_AGE);
   let rft_cookie = build_cookie("rft", refresh_token, COOKIE_MAX_AGE);

   // 5. Add cookie in headers
   cookies.add(act_cookie);
   cookies.add(rft_cookie);

   // 6. Response
   Ok((
      StatusCode::OK,
      Json(json!({
         "message": "Login successfully."
      })),
   ))
}

async fn logout(cookies: Cookies) -> impl IntoResponse {
   let expired_time = time::Duration::seconds(0);
   let expired_access = build_cookie("act", "".to_string(), expired_time);
   let expired_refresh = build_cookie("rft", "".to_string(), expired_time);

   cookies.remove(expired_access);
   cookies.remove(expired_refresh);

   (
      StatusCode::OK,
      Json(json!({
         "message": "Logged out successfully."
      })),
   )
}

async fn refresh_token(
   State(state): State<AppState>,
   cookies: Cookies,
) -> Result<impl IntoResponse, AppError> {
   let refresh_token = cookies
      .get("rft")
      .ok_or(AppError::Unauthorized(
         "Refresh token not found".to_string(),
      ))?
      .value()
      .to_string();

   let access_token = state.jwt_service.refresh(&refresh_token, ACCESS_TOKEN_TIME)?;

   let act_cookie = build_cookie("act", access_token, COOKIE_MAX_AGE);

   cookies.add(act_cookie);

   Ok((
      StatusCode::OK,
      Json(json!({
         "message": "Refresh token successfully."
      })),
   ))
}

fn build_cookie<'c>(key: &'c str, value: String, max_age: time::Duration) -> Cookie<'c> {
   // must add secure method later for https protocols
   Cookie::build((key, value))
      .path("/")
      .same_site(cookie::SameSite::Lax)
      .http_only(true)
      .max_age(max_age)
      .build()
}
