use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};

use crate::{ApiResponse, Ctx};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
}

/// Health check endpoint
pub async fn health_check(ctx: axum::extract::State<Ctx>) -> Json<ApiResponse<HealthResponse>> {
    // Test database connection
    let db_status = match sqlx::query("SELECT 1").fetch_one(&ctx.db).await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    Json(ApiResponse {
        code: 200,
        message: "OK".to_string(),
        data: Some(HealthResponse {
            status: "healthy".to_string(),
            database: db_status.to_string(),
        }),
    })
}

/// Mount health routes
pub fn mount() -> Router<Ctx> {
    Router::new().route("/health", get(health_check))
}
