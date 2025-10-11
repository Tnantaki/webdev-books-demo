use thiserror::Error;

pub mod books;
pub mod images;
pub mod users;

#[derive(Error, Debug)]
pub enum InMemError {
   #[error("{0}")]
   DataNotFound(String),
   
   #[error("Lock poisoned")]
   LockPoisoned,
}