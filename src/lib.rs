use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use error::AyiahError;
use serde::{Deserialize, Serialize};

pub mod context;
pub mod error;
pub mod migration;

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
