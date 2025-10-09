pub mod config;
pub mod server;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError<'a> {
   #[error("Env error: {0}")]
   EnvError(&'a str),
   
   #[error("Server error: {0}")]
   RunServerError(String),
}
