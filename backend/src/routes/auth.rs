use axum::{
   Json, Router,
   extract::State,
   http::StatusCode,
   middleware,
   response::IntoResponse,
   routing::{get, post},
};
use chrono::{Duration, TimeDelta};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{
   Cookie, Cookies,
   cookie::{self, time},
};
use validator::Validate;

use crate::{
   routes::{
      JsonResult,
      app_error::AppError,
      middleware::{auth_cookie, unwrap_cookie},
   },
   schemas::user::RegisterUser,
   services::{jwt_token::Token, password_hashing::PasswordService},
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

pub fn router(state: &AppState) -> Router<AppState> {
   Router::new()
      .route("/me", get(get_user_info))
      .route_layer(middleware::from_fn_with_state(
         state.jwt_service.clone(),
         auth_cookie,
      ))
      .route("/signup", post(signup))
      .route("/login", post(login))
      .route("/logout", post(logout))
      .route("/refresh", post(refresh_token))
}

async fn signup(
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
   state.postgres.user_repo.add_user(payload.email, password_hash).await?;

   // 5. Response
   Ok((
      StatusCode::CREATED,
      Json(json!({"message": "register user successfully."})),
   ))
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

async fn get_user_info(State(state): State<AppState>, cookies: Cookies) -> JsonResult<Token> {
   let access_token = unwrap_cookie(&cookies, "act")?;
   let auth_token = state.jwt_service.verify(&access_token)?;

   Ok((StatusCode::OK, Json(auth_token)))
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
      .same_site(cookie::SameSite::None)
      .http_only(true)
      .max_age(max_age)
      .build()
}
