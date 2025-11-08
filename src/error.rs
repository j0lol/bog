use std::fmt;

#[allow(unused_imports)]
use log::{debug, error, info, warn};
use poem::{Response, error::ResponseError, http::StatusCode};

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

impl ResponseError for AppError {
    fn status(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn as_response(&self) -> Response {
        let body = match self {
            AppError::NotFound => "404 Not Found",
            AppError::Unauthorized => "Unauthorized",
            _ => "Internal server error",
        };

        Response::builder().status(self.status()).body(body)
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

impl From<typst_as_lib::TypstAsLibError> for AppError {
    fn from(err: typst_as_lib::TypstAsLibError) -> Self {
        AppError::internal_server_error(format!("Typst error: {err}"))
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

pub type Result<T> = poem::Result<T, AppError>;
