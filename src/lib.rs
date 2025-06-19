#![forbid(unsafe_code)]

use std::sync::Arc;

use app::config::ConfigManager;
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use error::AyiahError;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod app;
pub mod db;
pub mod entities;
pub mod error;
pub mod middleware;
pub mod routes;
pub mod scraper;
pub mod utils;

pub type ApiResult<T> = std::result::Result<ApiResponse<T>, AyiahError>;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
        Json(self).into_response()
    }
}

/// AppState holds all shared application resources
#[derive(Clone)]
pub struct Context {
    /// Shared configuration manager
    pub config: ConfigManager,

    /// Database connection
    pub db: DatabaseConnection,
}

pub type Ctx = Arc<Context>;
