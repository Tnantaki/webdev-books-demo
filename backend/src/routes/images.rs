use axum::{
   extract::{Multipart, Path, State}, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get, post}, Json, Router
};
use uuid::Uuid;

use crate::{
   routes::{app_error::AppError, middleware::auth_cookie_admin, JsonResult},
   schemas::image::{AddImage, ImageResponse},
   startup::app_state::AppState,
};

pub fn router(state: AppState) -> Router {
   Router::new()
      .route("/", post(upload_image))
      .route("/{id}", delete(delete_image))
      .route_layer(middleware::from_fn_with_state(
         state.jwt_service.clone(),
         auth_cookie_admin,
      ))
      .route("/{id}", get(get_image))
      .with_state(state)
}

async fn upload_image(
   State(state): State<AppState>,
   mut multipart: Multipart,
) -> JsonResult<ImageResponse> {
   let field =
      multipart.next_field().await?.ok_or(AppError::Multipart("No file provided".to_string()))?;

   let name = field.name().unwrap_or("");
   if name != "image" {
      return Err(AppError::Multipart(
         "Expected field name 'image'".to_string(),
      ));
   }

   let content_type = field.content_type().unwrap_or("").to_string();
   if !content_type.starts_with("image/") {
      return Err(AppError::Multipart("file is not an image type".to_string()));
   }

   let filename = field.file_name().unwrap_or("unknown").to_string();
   let data = field.bytes().await.map_err(|e| AppError::Multipart(e.to_string()))?;

   let new_image = AddImage {
      filename: filename.clone(),
      content_type,
      data,
   };

   let id = state.image_repo.save_image(new_image)?;
   Ok((
      StatusCode::OK,
      Json(ImageResponse {
         id,
         filename,
         url: format!("/images/{}", id),
      }),
   ))
}

async fn get_image(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
   let image = state.image_repo.get_image(id)?;

   Ok((
      [(axum::http::header::CONTENT_TYPE, image.content_type)],
      image.data,
   ))
}

async fn delete_image(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
   state.image_repo.delete_image(id)?;

   Ok(StatusCode::NO_CONTENT)
}
