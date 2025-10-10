use axum::{
   Json, Router,
   extract::{Multipart, Path, State},
   http::StatusCode,
   response::IntoResponse,
   routing::{delete, get, post},
};
use uuid::Uuid;

use crate::{
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
) -> impl IntoResponse {
   let field = multipart
      .next_field()
      .await
      .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()).into_response())?
      .ok_or((StatusCode::BAD_REQUEST, "No file provided".to_string()).into_response())?;

   let filename = field.file_name().unwrap_or("unknown").to_string();
   let content_type = field.content_type().unwrap_or("image/jpeg").to_string();
   let data = field.bytes().await.unwrap();

   let new_image = AddImage {
      filename: filename.clone(),
      content_type,
      data,
   };

   if let Some(id) = state.image_repo.save_image(new_image) {
      Ok(Json(ImageResponse {
         id,
         filename,
         url: format!("/images/{}", id),
      }))
   } else {
      Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
   }
}

async fn get_image(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
   if let Some(image) = state.image_repo.get_image(id) {
      let res = (
         [(axum::http::header::CONTENT_TYPE, image.content_type)],
         image.data,
      );
      Ok(res)
   } else {
      Err(StatusCode::NOT_FOUND)
   }
}

async fn delete_image(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
   if state.image_repo.delete_image(id).is_some() {
      (StatusCode::NO_CONTENT).into_response()
   } else {
      (StatusCode::NOT_FOUND).into_response()
   }
}
