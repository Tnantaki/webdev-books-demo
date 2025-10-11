pub mod models;
pub mod repos;
pub mod routes;
pub mod schemas;
pub mod services;
pub mod startup;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError<'a> {
   #[error("Env error: {0}")]
   EnvError(&'a str),

   #[error("Server error: {0}")]
   RunServerError(String),

   #[error("Database connection error: {0}")]
   DatabaseError(String),
}
