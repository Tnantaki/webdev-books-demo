use axum::{
   Json, Router,
   extract::{DefaultBodyLimit, Multipart, Path, State},
   http::StatusCode,
   middleware,
   response::IntoResponse,
   routing::{delete, get, post},
};
use uuid::Uuid;

use crate::{
   routes::{JsonResult, app_error::AppError, middleware::auth_cookie_admin},
   schemas::image::{AddImage, ImageResponse, ImgType},
   startup::app_state::AppState,
};

const FIELD_NAME_IMAGE: &str = "image";
pub const IMAGE_LIMIT_TEXT: &str = "5MB";
const DEFAULT_IMAGE_LIMIT: usize = 5 * 1024 * 1024;

pub fn router(state: &AppState) -> Router<AppState> {
   Router::new()
      .route("/", post(upload_image))
      .layer(DefaultBodyLimit::max(DEFAULT_IMAGE_LIMIT))
      .route("/{id}", delete(delete_image))
      .route_layer(middleware::from_fn_with_state(
         state.jwt_service.clone(),
         auth_cookie_admin,
      ))
      .route("/{id}", get(get_image_by_id))
}

async fn upload_image(
   State(state): State<AppState>,
   mut multipart: Multipart,
) -> JsonResult<ImageResponse> {
   let field = multipart
      .next_field() // get only first field, loop to get multiple field
      .await? // check request type (multipart data), network while reading
      .ok_or(AppError::UploadFile("No file provided".to_string()))?; // check field is exist

   let field_name = field.name().unwrap_or("");
   if field_name != FIELD_NAME_IMAGE {
      return Err(AppError::UploadFile(format!(
         "Expected field name {FIELD_NAME_IMAGE}"
      )));
   }

   // Validate is content type support
   let content_type = field.content_type().unwrap_or("").to_string();
   let img_type = ImgType::from_content_type(&content_type)?;

   let filename = field.file_name().unwrap_or("unknown").to_string();

   // Validate file size (max 5MB)
   let data = field.bytes().await?.to_vec();

   let add_image = AddImage::new(filename, img_type, data);
   let image_metadata = state.postgres.image_repo.save_image(add_image).await?;

   Ok((StatusCode::OK, Json(ImageResponse::from(image_metadata))))
}

async fn get_image_by_id(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
   let image = state.postgres.image_repo.get_image_by_id(id).await?;

   Ok((
      [(axum::http::header::CONTENT_TYPE, image.content_type)],
      image.data,
   ))
}

async fn delete_image(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
   state.postgres.image_repo.delete_image(id).await?;

   Ok(StatusCode::NO_CONTENT)
}
