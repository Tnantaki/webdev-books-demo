use axum::{
   Json, Router,
   extract::{Path, State},
   http::StatusCode,
   middleware,
   routing::{delete, get, patch, post},
};
use uuid::Uuid;
use validator::Validate;

use crate::{
   routes::{JsonResult, app_error::AppError, middleware::auth_cookie_admin},
   schemas::book::{AddBook, Book, EditBook},
   startup::app_state::AppState,
};

pub fn router(state: AppState) -> Router {
   Router::new()
      .route("/", post(add_book))
      .route("/{id}", patch(edit_book_by_id))
      .route("/{id}", delete(delete_book_by_id))
      .route_layer(middleware::from_fn_with_state(
         state.jwt_service.clone(),
         auth_cookie_admin,
      ))
      .route("/", get(view_books))
      .route("/{id}", get(view_book_by_id))
      .with_state(state)
}

async fn add_book(
   State(state): State<AppState>,
   Json(payload): Json<AddBook>,
   // ) -> Result<(StatusCode, Json<Book>), AppError> {
) -> JsonResult<Book> {
   payload.validate()?;

   let book = state.book_repo.add_book(payload)?;

   Ok((StatusCode::CREATED, Json(book)))
}

async fn view_books(State(state): State<AppState>) -> JsonResult<Vec<Book>> {
   let books = state.book_repo.view_books()?;

   Ok((StatusCode::OK, Json(books)))
}

async fn view_book_by_id(State(state): State<AppState>, Path(id): Path<Uuid>) -> JsonResult<Book> {
   let book = state.book_repo.view_book_by_id(id)?;

   Ok((StatusCode::OK, Json(book)))
}

async fn edit_book_by_id(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
   Json(payload): Json<EditBook>,
) -> JsonResult<Book> {
   let book = state.book_repo.edit_book(id, payload)?;

   Ok((StatusCode::OK, Json(book)))
}

async fn delete_book_by_id(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
   state.book_repo.delete_book(id)?;

   Ok(StatusCode::NO_CONTENT)
}
