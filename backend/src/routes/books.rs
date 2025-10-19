use axum::{
   Json, Router,
   extract::{Path, Query, State},
   http::StatusCode,
   middleware,
   routing::{delete, get, patch, post},
};
use uuid::Uuid;
use validator::Validate;

use crate::{
   models::books::BookFilter,
   routes::{JsonResult, app_error::AppError, middleware::auth_cookie_admin},
   schemas::book::{AddBook, Book, EditBook, PaginationMeta, PaginationParams, PaginationResponse},
   startup::app_state::AppState,
};

pub fn router(state: &AppState) -> Router<AppState> {
   Router::new()
      .route("/", post(add_book))
      .route("/{id}", patch(edit_book_by_id))
      .route("/{id}", delete(delete_book_by_id))
      .route_layer(middleware::from_fn_with_state(
         state.jwt_service.clone(),
         auth_cookie_admin,
      ))
      .route("/", get(view_books))
      .route("/page", get(get_book_pages))
      .route("/{id}", get(view_book_by_id))
}

async fn add_book(State(state): State<AppState>, Json(payload): Json<AddBook>) -> JsonResult<Book> {
   payload.validate()?;

   // Checking is image exist in database
   state.postgres.image_repo.get_image_by_id(payload.image_id).await?;

   let book = state.postgres.book_repo.add_book(payload).await?;

   Ok((StatusCode::CREATED, Json(book)))
}

async fn view_books(State(state): State<AppState>) -> JsonResult<Vec<Book>> {
   let books = state.postgres.book_repo.get_all_book().await?;

   Ok((StatusCode::OK, Json(books)))
}

async fn view_book_by_id(State(state): State<AppState>, Path(id): Path<Uuid>) -> JsonResult<Book> {
   let book = state.postgres.book_repo.get_book_by_id(id).await?;

   Ok((StatusCode::OK, Json(book)))
}

async fn edit_book_by_id(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
   Json(payload): Json<EditBook>,
) -> JsonResult<Book> {
   if let Some(image_id) = payload.image_id {
      state.postgres.image_repo.get_image_by_id(image_id).await?;
   }

   let book = state.postgres.book_repo.edit_book(id, payload).await?;

   Ok((StatusCode::OK, Json(book)))
}

async fn delete_book_by_id(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
   state.postgres.book_repo.delete_book(id).await?;

   Ok(StatusCode::NO_CONTENT)
}

// validate image existing in database by image path
#[allow(dead_code)]
async fn validate_exist_image(img_path: &str, state: &AppState) -> Result<(), AppError> {
   // Parsing uuid from image path
   let image_id = img_path
      .strip_prefix("/images/")
      .and_then(|image_str_id| Uuid::parse_str(image_str_id).ok())
      .ok_or_else(|| AppError::BadRequest("Invalid image url path".to_string()))?;

   // Checking is image exist in database
   state.postgres.image_repo.get_image_by_id(image_id).await?;

   Ok(())
}

async fn get_book_pages(
   State(state): State<AppState>,
   Query(params): Query<PaginationParams>,
) -> JsonResult<PaginationResponse<Book>> {
   println!("debug books params: {:?}", params); // DEBUG

   let page = params.page;
   let per_page = params.per_page;
   let offset = (page - 1) * per_page;
   let sort_by = params.sort_by;
   let order = params.order;
   let genre = params.genre;

   let book_filter = BookFilter::new(Some(per_page), sort_by, order, genre, Some(offset));
   let (total_items, books) = state.postgres.book_repo.get_all_book_filter(book_filter).await?;

   // Calculate pagination metadata
   let total_pages = (total_items as f64 / per_page as f64).ceil() as i64;
   let has_next = page < total_pages;
   let has_previous = page > 1;

   let response = PaginationResponse {
      data: books,
      pagination: PaginationMeta {
         current_page: page,
         per_page,
         total_items,
         total_pages,
         has_next,
         has_previous,
      },
   };

   Ok((StatusCode::OK, Json(response)))
}
