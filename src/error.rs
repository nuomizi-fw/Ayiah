use axum::{
    Json,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum AyiahError {
    #[error("{0}")]
    ApiError(#[from] ApiError),

    #[error("{0}")]
    AuthError(#[from] AuthError),

    #[error("{0}")]
    ConfigError(#[from] ConfigError),

    // #[error("{0}")]
    // DatabaseError(String),

    // #[error("{0}")]
    // SqlxError(#[from] sqlx::Error),

    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("{0}")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("{0}")]
    ScrapeError(#[from] ScrapeError),
}

impl AyiahError {
    fn code(&self) -> (StatusCode, String) {
        match self {
            Self::ApiError(err) => err.code(),
            Self::AuthError(err) => err.code(),
            Self::ConfigError(err) => err.code(),
            // Self::DatabaseError(err) => {
            //     tracing::error!("Database error: {}", err);
            //     (
            //         StatusCode::INTERNAL_SERVER_ERROR,
            //         "An error occurred while accessing the database".to_string(),
            //     )
            // }
            // Self::SqlxError(err) => {
            //     tracing::error!("SQLx error: {}", err);
            //     (
            //         StatusCode::INTERNAL_SERVER_ERROR,
            //         "A database error occurred".to_string(),
            //     )
            // }
            Self::SerdeJsonError(err) => {
                tracing::error!("Serde JSON error: {}", err);
                (StatusCode::BAD_REQUEST, "Invalid JSON format".to_string())
            }
            Self::ValidationError(err) => (
                StatusCode::BAD_REQUEST,
                format!("Validation error: {}", err),
            ),
            Self::ScrapeError(err) => {
                tracing::error!("Scrape error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Scrape operation failed: {}", err),
                )
            }
        }
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

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,

    #[error("Token creation failed")]
    TokenCreation,

    #[error("Missing authentication")]
    MissingAuth,
}

impl AuthError {
    fn code(&self) -> (StatusCode, String) {
        match self {
            Self::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "Invalid authentication token".to_string(),
            ),
            Self::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create authentication token".to_string(),
            ),
            Self::MissingAuth => (
                StatusCode::UNAUTHORIZED,
                "Authentication required".to_string(),
            ),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Failed to load configuration: {0}")]
    LoadError(#[from] config::ConfigError),

    #[error("Failed to parse configuration: {0}")]
    ParseError(String),

    #[error("Failed to write configuration: {0}")]
    WriteError(String),

    #[error("Configuration not initialized")]
    NotInitialized,
}

impl ConfigError {
    fn code(&self) -> (StatusCode, String) {
        match self {
            Self::LoadError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to load configuration: {}", err),
            ),
            Self::ParseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse configuration: {}", msg),
            ),
            Self::WriteError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to write configuration: {}", msg),
            ),
            Self::NotInitialized => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Configuration not initialized".to_string(),
            ),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ScrapeError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("File type not supported: {0}")]
    UnsupportedFileType(String),

    #[error("Metadata fetch failed: {0}")]
    MetadataFetchError(String),

    #[error("File organization failed: {0}")]
    OrganizationError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Symlink creation failed: {0}")]
    SymlinkError(String),

    #[error("Hard link creation failed: {0}")]
    HardLinkError(String),

    #[error("Copy operation failed: {0}")]
    CopyError(String),

    #[error("Move operation failed: {0}")]
    MoveError(String),

    #[error("Path already exists: {0}")]
    PathExists(String),

    #[error("Directory creation failed: {0}")]
    DirectoryCreationError(String),

    #[error("File scan failed: {0}")]
    ScanError(String),

    #[error("Task join error: {0}")]
    TaskJoinError(#[from] tokio::task::JoinError),

    #[error("Channel send error")]
    ChannelSendError,

    #[error("Channel receive error")]
    ChannelReceiveError,
}
