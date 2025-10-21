use poem::{IntoResponse, Response, http::StatusCode};
use std::fmt;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

pub enum AppError {
    NotFound,
    Unauthorized,
    DatabaseError(String),
    InternalServerError(String),
}

impl AppError {
    pub fn internal_server_error(msg: String) -> Self {
        error!("Internal server error: {msg}");
        AppError::InternalServerError(msg)
    }

    pub fn database_error(msg: String) -> Self {
        error!("Database error: {msg}");
        AppError::DatabaseError(msg)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Not found"),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::DatabaseError(msg) => write!(f, "Database error: {msg}"),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {msg}"),
        }
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            AppError::DatabaseError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
            AppError::InternalServerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        match err {
            rusqlite::Error::QueryReturnedNoRows => AppError::NotFound,
            _ => AppError::database_error(err.to_string()),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::internal_server_error(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::internal_server_error(err.to_string())
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::internal_server_error(err)
    }
}

impl<T> From<std::sync::PoisonError<T>> for AppError {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        AppError::internal_server_error("Lock poisoned".to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
