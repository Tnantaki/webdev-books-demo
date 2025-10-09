use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use uuid::Uuid;

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
#[derive(Serialize)]
pub struct Book {
   pub id: Uuid,
   pub title: String,
   pub description: String,
}

pub fn routes() -> Router {
   Router::new().route("/", get(view_books))
}

async fn view_books() -> impl IntoResponse {
   let books = vec![
      Book {
         id: Uuid::now_v7(),
         title: "C++".to_string(),
         description: "OOP programming language".to_string(),
      },
      Book {
         id: Uuid::now_v7(),
         title: "Rust".to_string(),
         description: "Secure programming language".to_string(),
      },
   ];

   (StatusCode::OK, Json(books))
}
