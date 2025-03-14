use axum::{
    Json,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use serde_json::json;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct AyiahError(#[from] ErrorKind);

impl AyiahError {
    pub fn code(&self) -> (StatusCode, String) {
        self.0.code()
    }
}

// Implement Axum's IntoResponse for our error type
impl IntoResponse for AyiahError {
    fn into_response(self) -> Response {
        let (status_code, message) = self.code();
        let body = Json(json!({
            "code": status_code.as_u16(),
            "message": message,
        }));

        (status_code, body).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("{0}")]
    ApiError(#[from] ApiError),

    #[error("{0}")]
    DbError(#[from] sea_orm::DbErr),

    #[error("{0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("{0}")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("{0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}

impl ErrorKind {
    fn code(&self) -> (StatusCode, String) {
        match self {
            Self::ApiError(err) => err.code(),
            Self::DbError(err) => {
                tracing::error!("Database error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An error occurred while accessing the database".to_string(),
                )
            }
            Self::BcryptError(err) => {
                tracing::error!("Bcrypt error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An error occurred while processing authentication".to_string(),
                )
            }
            Self::SerdeJsonError(err) => {
                tracing::error!("Serde JSON error: {}", err);
                (StatusCode::BAD_REQUEST, "Invalid JSON format".to_string())
            }
            Self::ValidationError(err) => (
                StatusCode::BAD_REQUEST,
                format!("Validation error: {}", err),
            ),
            Self::JwtError(err) => {
                tracing::error!("JWT error: {}", err);
                (
                    StatusCode::UNAUTHORIZED,
                    "Authentication token error".to_string(),
                )
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    Unauthorized(String),

    #[error("{0}")]
    Forbidden(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Conflict(String),

    #[error("{0}")]
    InternalServerError(String),
}

impl ApiError {
    fn code(&self) -> (StatusCode, String) {
        match self {
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            Self::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            Self::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            Self::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            Self::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        }
    }
}
