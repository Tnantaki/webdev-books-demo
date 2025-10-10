use axum::{
   Json, Router,
   extract::{Path, State},
   http::StatusCode,
   response::IntoResponse,
   routing::{delete, get, patch, post},
};
use uuid::Uuid;
use validator::Validate;

use crate::{
   schemas::book::{AddBook, EditBook},
   startup::app_state::AppState,
};

pub fn router(state: AppState) -> Router {
   Router::new()
      .route("/", post(add_book))
      .route("/", get(view_books))
      .route("/{id}", get(view_book_by_id))
      .route("/{id}", patch(edit_book_by_id))
      .route("/{id}", delete(delete_book_by_id))
      .with_state(state)
}

async fn add_book(
   State(state): State<AppState>,
   Json(payload): Json<AddBook>,
) -> impl IntoResponse {
   if let Err(e) = payload.validate() {
      return (StatusCode::BAD_REQUEST, e.to_string()).into_response();
   }

   let book = state.book_repo.add_book(payload);

   (StatusCode::CREATED, Json(book)).into_response()
}

async fn view_books(State(state): State<AppState>) -> impl IntoResponse {
   let books = state.book_repo.view_books();

   (StatusCode::OK, Json(books))
}

async fn view_book_by_id(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
   if let Some(book) = state.book_repo.view_book_by_id(id) {
      (StatusCode::OK, Json(book)).into_response()
   } else {
      (StatusCode::NOT_FOUND).into_response()
   }
}

async fn edit_book_by_id(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
   Json(payload): Json<EditBook>,
) -> impl IntoResponse {
   if let Some(book) = state.book_repo.edit_book(id, payload) {
      (StatusCode::OK, Json(book)).into_response()
   } else {
      (StatusCode::NOT_FOUND).into_response()
   }
}

async fn delete_book_by_id(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
) -> impl IntoResponse {
   if state.book_repo.delete_book(id).is_some() {
      (StatusCode::NO_CONTENT).into_response()
   } else {
      (StatusCode::NOT_FOUND).into_response()
   }
}
