use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::routes::books::Book;

// in-memory storage (temporary)
#[derive(Clone)]
pub struct AppState {
   pub books: Arc<Mutex<Vec<Book>>>,
}

impl AppState {
   pub fn new() -> Self {
      let books = mockup_books();
      
      Self {
         books: Arc::new(Mutex::new(books)),
      }
   }
}

fn mockup_books() -> Vec<Book> {
   vec![
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
   ]
}
