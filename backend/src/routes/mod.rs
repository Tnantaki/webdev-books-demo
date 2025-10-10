use crate::routes::app_error::AppError;
use axum::{http::StatusCode, Json};

pub type JsonResult<T, E = AppError> = std::result::Result<(StatusCode, Json<T>), E>;

pub mod app_error;
pub mod books;
pub mod images;

