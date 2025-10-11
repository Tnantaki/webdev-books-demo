use axum::{
   Json,
   extract::multipart::MultipartError,
   http::StatusCode,
   response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;
use validator::{ValidationErrors, ValidationErrorsKind};

use crate::{repos::in_mem::InMemError, services::password_hashing::PasswordHashError};

#[derive(Error, Debug)]
pub enum AppError {
   #[error("Validation error")]
   Validation(ValidationErrors),

   #[error("Bad request error: {0}")]
   BadRequest(String),

   #[error("Conflict error: {0}")]
   Conflict(String),

   #[error("Upload file error: {0}")]
   Multipart(String),

   #[error("Not found error: {0}")]
   NotFound(String),

   #[error("Not found error: {0}")]
   Unauthorized(String),

   #[error("Intenal server error: {0}")]
   InMemError(String),

   #[error("Intenal server error: {0}")]
   InternalError(String),
}

impl AppError {
   pub fn status_code(&self) -> StatusCode {
      match self {
         // Validation errors -> 400
         AppError::Validation(_) | AppError::Multipart(_) | AppError::BadRequest(_) => {
            StatusCode::BAD_REQUEST
         }

         // Unauthorized errors -> 401
         AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,

         // Forbidden errors -> 403

         // Not found errors -> 404
         AppError::NotFound(_) => StatusCode::NOT_FOUND,

         // Conflict errors -> 409
         AppError::Conflict(_) => StatusCode::CONFLICT,

         // Database errors -> 500
         AppError::InMemError(_) | AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      }
   }

   pub fn error_message(&self) -> String {
      self.to_string()
   }
}

#[derive(Serialize)]
pub struct ErrorResponse {
   pub error: String,
   #[serde(skip_serializing_if = "Option::is_none")]
   pub fields: Option<Vec<FieldError>>,
}

impl IntoResponse for AppError {
   fn into_response(self) -> Response {
      let status_code = self.status_code();
      let error_message = self.error_message();

      if status_code == StatusCode::INTERNAL_SERVER_ERROR {
         eprintln!("{error_message}"); // debug
         return StatusCode::INTERNAL_SERVER_ERROR.into_response();
      }

      let mut fields = None;
      if let AppError::Validation(errors) = self {
         let field_errors = format_validation_errors(&errors);
         fields = Some(field_errors);
      }

      let body = Json(ErrorResponse {
         error: error_message,
         fields,
      });

      (status_code, body).into_response()
   }
}

impl From<ValidationErrors> for AppError {
   fn from(error: ValidationErrors) -> Self {
      AppError::Validation(error)
   }
}

impl From<MultipartError> for AppError {
   fn from(error: MultipartError) -> Self {
      AppError::Multipart(error.to_string())
   }
}

impl From<PasswordHashError> for AppError {
   fn from(error: PasswordHashError) -> Self {
      match error {
         PasswordHashError::VerifyFail(msg) => AppError::Unauthorized(msg),
         PasswordHashError::HashFail(msg) => AppError::InternalError(msg),
      }
   }
}

impl From<InMemError> for AppError {
   fn from(error: InMemError) -> Self {
      match error {
         InMemError::DataNotFound(msg) => AppError::NotFound(msg),
         InMemError::LockPoisoned => AppError::InternalError(error.to_string()),
      }
   }
}

#[derive(Serialize)]
pub struct FieldError {
   pub field: String,
   pub message: String,
   #[serde(skip_serializing_if = "Option::is_none")]
   pub params: Option<HashMap<String, Value>>,
}

fn format_validation_errors(errors: &ValidationErrors) -> Vec<FieldError> {
   let mut field_errors = Vec::new();

   for (field, error_kind) in errors.errors() {
      if let ValidationErrorsKind::Field(field_error_list) = error_kind {
         for error in field_error_list {
            let code = error.code.as_ref();
            let message = error.message.as_deref().unwrap_or(code).to_string();
            let params = error.params.iter().map(|(k, v)| (k.to_string(), v.clone())).collect();

            field_errors.push(FieldError {
               field: field.to_string(),
               message,
               params: Some(params),
            });
         }
      }
   }

   field_errors
}
