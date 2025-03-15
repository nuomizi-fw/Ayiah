use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use error::AyiahError;
use serde::{Deserialize, Serialize};

pub mod config;
pub mod context;
pub mod entity;
pub mod error;
pub mod graceful_shutdown;
pub mod logging;
pub mod middleware;
pub mod migration;
pub mod routes;

pub type ApiResult<T> = std::result::Result<ApiResponse<T>, AyiahError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = Json(self);

        (StatusCode::OK, body).into_response()
    }
}
