use crate::routes::app_error::AppError;
use axum::{http::StatusCode, Json};

pub type JsonResult<T, E = AppError> = std::result::Result<(StatusCode, Json<T>), E>;

pub mod app_error;
pub mod books;
pub mod images;
pub mod users;
pub mod auth;
pub mod middleware;
pub mod cart_items;

