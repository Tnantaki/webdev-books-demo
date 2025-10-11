use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use chrono::Duration;
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{Cookie, Cookies, cookie};

use crate::{
   routes::app_error::AppError, services::password_hashing::PasswordService,
   startup::app_state::AppState,
};

pub fn router(state: AppState) -> Router {
   Router::new().route("/login", post(login)).with_state(state)
}

#[derive(Deserialize)]
pub struct Login {
   email: String,
   password: String,
}

async fn login(
   State(state): State<AppState>,
   cookies: Cookies,
   Json(payload): Json<Login>,
) -> Result<impl IntoResponse, AppError> {
   // 1. Find email in repo
   let user = state
      .user_repo
      .get_user_by_email(&payload.email)
      .map_err(|_| AppError::Unauthorized("Invalid email".to_string()))?;

   // 2. Verrify user by email & password
   PasswordService::verify(&payload.password, &user.password_hash)?;

   // 3. Generate token
   let access_token = state.jwt_service.generate(&user, Duration::minutes(30))?;
   let refresh_token = state.jwt_service.generate(&user, Duration::days(14))?;

   // 4. Build cookie by token
   let act_cookie = build_cookie("act", access_token);
   let rft_cookie = build_cookie("rft", refresh_token);

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

fn build_cookie<'c>(key: &'c str, value: String) -> Cookie<'c> {
   // must add secure method later for https protocols
   Cookie::build((key, value))
      .path("/")
      .same_site(cookie::SameSite::Lax)
      .http_only(true)
      .max_age(cookie::time::Duration::days(14))
      .build()
}
