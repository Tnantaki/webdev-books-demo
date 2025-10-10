use axum::{
   Json, Router,
   extract::{Multipart, Path, State},
   http::StatusCode,
   response::IntoResponse,
   routing::{delete, get, post},
};
use uuid::Uuid;

use crate::{
   routes::{JsonResult, app_error::AppError},
   schemas::image::{AddImage, ImageResponse},
   startup::app_state::AppState,
};

pub fn router(state: AppState) -> Router {
   Router::new()
      .route("/", post(upload_image))
      .route("/{id}", get(get_image))
      .route("/{id}", delete(delete_image))
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

   if let Some(id) = state.image_repo.save_image(new_image) {
      Ok((
         StatusCode::OK,
         Json(ImageResponse {
            id,
            filename,
            url: format!("/images/{}", id),
         }),
      ))
   } else {
      Err(AppError::InternalError)
   }
}

async fn get_image(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
   if let Some(image) = state.image_repo.get_image(id) {
      let res = (
         [(axum::http::header::CONTENT_TYPE, image.content_type)],
         image.data,
      );
      Ok(res)
   } else {
      Err(AppError::NotFound("not found image by id".to_string()))
   }
}

async fn delete_image(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
   if state.image_repo.delete_image(id).is_some() {
      Ok(StatusCode::NO_CONTENT)
   } else {
      Err(AppError::NotFound("not found image by id".to_string()))
   }
}
