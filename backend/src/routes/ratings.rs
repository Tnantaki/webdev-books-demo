use axum::{
   Extension, Json, Router,
   extract::{Path, State},
   http::StatusCode,
   middleware,
   routing::{get, post},
};
use uuid::Uuid;
use validator::Validate;

use crate::{
   models::ratings::{BookRatingModel, RatingModel},
   routes::{JsonResult, middleware::auth_cookie},
   schemas::rating::GiveRate,
   startup::app_state::AppState,
};

pub fn router(state: &AppState) -> Router<AppState> {
   Router::new()
      .route("/{book_id}/rating", post(upsert_book_rate))
      .route_layer(middleware::from_fn_with_state(
         state.jwt_service.clone(),
         auth_cookie,
      ))
      .route("/{book_id}/rating", get(get_book_ratings))
}

async fn upsert_book_rate(
   State(state): State<AppState>,
   Extension(user_id): Extension<Uuid>,
   Path(book_id): Path<Uuid>,
   Json(payload): Json<GiveRate>,
) -> JsonResult<RatingModel> {
   payload.validate()?;

   let rate = state.postgres.rating_repo.upsert_rating(user_id, book_id, payload).await?;

   Ok((StatusCode::OK, Json(rate)))
}

async fn get_book_ratings(
   State(state): State<AppState>,
   Path(book_id): Path<Uuid>,
) -> JsonResult<Vec<BookRatingModel>> {
   let book_ratings = state.postgres.rating_repo.get_book_rating(book_id).await?;

   Ok((StatusCode::OK, Json(book_ratings)))
}
