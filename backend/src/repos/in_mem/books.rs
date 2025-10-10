use crate::{
   models::books::BookModel,
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

   pub fn add_book(&self, new_book: AddBook) -> Book {
      let book = BookModel::add(new_book);
      {
         let mut books = self.books.lock().unwrap();
         books.push(book.clone());
      }
      Book::from(book)
   }

   pub fn view_books(&self) -> Vec<Book> {
      let books = self.books.lock().unwrap().clone();

      books.into_iter().map(|book| Book::from(book)).collect()
   }

   pub fn view_book_by_id(&self, id: Uuid) -> Option<Book> {
      let books = self.books.lock().unwrap();

      books.iter().find(|book| book.id == id).cloned().map(Book::from)
   }

   pub fn edit_book(&self, id: Uuid, edit_book: EditBook) -> Option<Book> {
      let mut books = self.books.lock().unwrap();

      books.iter_mut().find(|book| book.id == id).map(|book| {
         book.edit(edit_book);
         return Book::from(book.clone());
      })
   }

   pub fn delete_book(&self, id: Uuid) -> Option<()> {
      let mut books = self.books.lock().unwrap();

      books.iter().position(|book| book.id == id).map(|idx| books.remove(idx)).map(|_| ())
   }
}
