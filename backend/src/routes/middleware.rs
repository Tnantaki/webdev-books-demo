use crate::{
   models::users::Role, routes::app_error::AppError, services::jwt_token::JwtTokenService,
};
use axum::{
   extract::{Request, State},
   middleware::Next,
   response::Response,
};
use tower_cookies::Cookies;

pub async fn auth_cookie(
   State(service): State<JwtTokenService>,
   cookies: Cookies,
   mut req: Request,
   next: Next,
) -> Result<Response, AppError> {
   let access_token = unwrap_cookie(&cookies, "act")?;
   let auth_token = service.verify(&access_token)?;

   // Inject auth token into request extensions
   req.extensions_mut().insert(auth_token.user_id);

   Ok(next.run(req).await)
}

pub async fn auth_cookie_admin(
   State(service): State<JwtTokenService>,
   cookies: Cookies,
   mut req: Request,
   next: Next,
) -> Result<Response, AppError> {
   let access_token = unwrap_cookie(&cookies, "act")?;
   let auth_token = service.verify(&access_token)?;

   // Verify Admin role
   if auth_token.role != Role::Admin {
      return Err(AppError::PermissionDenied);
   }
   // Inject auth token into request extensions
   req.extensions_mut().insert(auth_token.user_id);

   Ok(next.run(req).await)
}

fn unwrap_cookie(cookies: &Cookies, name: &str) -> Result<String, AppError> {
   let access_token = cookies
      .get(name)
      .ok_or(AppError::Unauthorized("Access token not found".to_string()))?
      .value()
      .to_string();
   Ok(access_token)
}
