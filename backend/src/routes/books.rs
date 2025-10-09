use axum::{
   Json, Router,
   extract::{Path, State},
   http::StatusCode,
   response::IntoResponse,
   routing::{delete, get, patch, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::startup::app_state::AppState;

// pub struct Book {
//    pub id: Uuid,
//    pub title: String,
//    pub genre: Genre,
//    pub description: String,
//    pub avaiable: i32,
//    pub price_in_pound: i32,
//    pub rating: i8,
//    pub img_path: String
// }

// For starter
#[derive(Serialize, Clone)]
pub struct Book {
   pub id: Uuid,
   pub title: String,
   pub description: String,
}

#[derive(Deserialize)]
pub struct AddBook {
   pub title: String,
   pub description: String,
}

#[derive(Deserialize)]
pub struct EditBook {
   pub title: Option<String>,
   pub description: Option<String>,
}

impl Book {
   fn new(add_book: AddBook) -> Self {
      Self {
         id: Uuid::now_v7(),
         title: add_book.title,
         description: add_book.description,
      }
   }

   fn edit(&mut self, edit_book: EditBook) {
      if let Some(title) = edit_book.title {
         self.title = title;
      }
      if let Some(description) = edit_book.description {
         self.description = description;
      }
   }
}

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
   let book = Book::new(payload);

   {
      let mut books = state.books.lock().unwrap();
      books.push(book.clone());
   }

   (StatusCode::CREATED, Json(book))
}

async fn view_books(State(state): State<AppState>) -> impl IntoResponse {
   let books = state.books.lock().unwrap().clone();

   (StatusCode::OK, Json(books))
}

async fn view_book_by_id(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
   let books = state.books.lock().unwrap();

   if let Some(book) = books.iter().find(|book| book.id == id) {
      (StatusCode::OK, Json(book.clone())).into_response()
   } else {
      (StatusCode::NOT_FOUND).into_response()
   }
}

async fn edit_book_by_id(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
   Json(payload): Json<EditBook>,
) -> impl IntoResponse {
   let mut books = state.books.lock().unwrap();

   if let Some(book) = books.iter_mut().find(|book| book.id == id) {
      book.edit(payload);
      (StatusCode::OK, Json(book.clone())).into_response()
   } else {
      (StatusCode::NOT_FOUND).into_response()
   }
}

async fn delete_book_by_id(
   State(state): State<AppState>,
   Path(id): Path<Uuid>,
) -> impl IntoResponse {
   let mut books = state.books.lock().unwrap();

   if let Some(idx) = books.iter().position(|book| book.id == id) {
      books.remove(idx);
      (StatusCode::NO_CONTENT).into_response()
   } else {
      (StatusCode::NOT_FOUND).into_response()
   }
}
