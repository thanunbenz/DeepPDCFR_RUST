use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;
use utoipa::ToSchema;

/// Application error type
#[derive(Debug)]
pub enum AppError {
    ValidationError(String),
    NotFound(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

/// Error response body matching Python API
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorDetail {
    pub error: String,
    pub message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let (error_code, message) = match self {
            AppError::ValidationError(msg) => ("validation_error", msg.clone()),
            AppError::NotFound(msg) => ("not_found", msg.clone()),
            AppError::Internal(msg) => ("internal_error", msg.clone()),
        };

        HttpResponse::build(self.status_code()).json(ErrorDetail {
            error: error_code.to_string(),
            message,
        })
    }
}

// Implement From<&str> for convenient error creation
impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        AppError::Internal(msg.to_string())
    }
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        AppError::Internal(msg)
    }
}
