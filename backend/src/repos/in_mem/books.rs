use crate::{
   models::books::BookModel,
   repos::in_mem::InMemError,
   schemas::book::{AddBook, Book, EditBook},
};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// cloning Arc doesn't clone the underlying data—it just creates another reference to the same data.
#[derive(Clone)]
pub struct BookRepo {
   books: Arc<Mutex<Vec<BookModel>>>,
}

impl BookRepo {
   pub fn new() -> Self {
      Self {
         books: Arc::new(Mutex::new(vec![])),
      }
   }

   pub fn add_book(&self, new_book: AddBook) -> Result<Book, InMemError> {
      let book = BookModel::add(new_book);
      {
         let mut books = self.books.lock().map_err(|_| InMemError::LockPoisoned)?;
         books.push(book.clone());
      }
      Ok(Book::from(book))
   }

   pub fn view_books(&self) -> Result<Vec<Book>, InMemError> {
      let books = self.books.lock().unwrap().clone();

      let books = books.into_iter().map(|book| Book::from(book)).collect();
      Ok(books)
   }

   pub fn view_book_by_id(&self, id: Uuid) -> Result<Book, InMemError> {
      let books = self.books.lock().map_err(|_| InMemError::LockPoisoned)?;

      books
         .iter()
         .find(|book| book.id == id)
         .cloned()
         .map(Book::from)
         .ok_or(InMemError::DataNotFound("invalid book id".to_string()))
   }

   pub fn edit_book(&self, id: Uuid, edit_book: EditBook) -> Result<Book, InMemError> {
      let mut books = self.books.lock().map_err(|_| InMemError::LockPoisoned)?;

      books
         .iter_mut()
         .find(|book| book.id == id)
         .map(|book| {
            book.edit(edit_book);
            return Book::from(book.clone());
         })
         .ok_or(InMemError::DataNotFound("invalid book id".to_string()))
   }

   pub fn delete_book(&self, id: Uuid) -> Result<(), InMemError> {
      let mut books = self.books.lock().map_err(|_| InMemError::LockPoisoned)?;

      books
         .iter()
         .position(|book| book.id == id)
         .map(|idx| books.remove(idx))
         .map(|_| ())
         .ok_or(InMemError::DataNotFound("invalid book id".to_string()))
   }
}
