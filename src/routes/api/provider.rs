use std::collections::HashMap;

use axum::{
    Json, Router,
    extract::Path,
    routing::{get, post},
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    ApiResponse, ApiResult, Ctx,
    error::{ApiError, AyiahError},
    scraper::MediaType,
};

#[derive(Debug, Deserialize, Validate)]
pub struct ProviderConnectionTestPayload {
    /// Timeout duration (seconds)
    #[validate(range(
        min = 1,
        max = 60,
        message = "Timeout must be between 1 and 60 seconds"
    ))]
    pub timeout_seconds: Option<u32>,
}

/// Providers response
#[derive(Debug, Serialize)]
pub struct ProvidersResponse {
    /// List of available providers
    pub providers: Vec<ProviderInfo>,
}

/// Provider information
#[derive(Debug, Serialize)]
pub struct ProviderInfo {
    /// Display name
    pub name: String,
    /// Supported media types
    pub supported_media_types: Vec<MediaType>,
    /// Whether API key is required
    pub requires_api_key: bool,
    /// Whether available
    pub available: bool,
}

pub fn mount() -> Router<Ctx> {
    Router::new().nest(
        "/provider",
        Router::new() // Get supported providers list
            .route("/", get(get_supported_providers))
            // Test provider connection
            .route("/{provider}/test", post(test_provider_connection)),
    )
}

pub async fn get_supported_providers() -> ApiResult<ProvidersResponse> {
    // TODO: Detect availability of each provider
    let response = ProvidersResponse { providers: vec![] };

    Ok(ApiResponse {
        code: StatusCode::OK.into(),
        message: "Providers list retrieved".to_string(),
        data: Some(response),
    })
}

pub async fn test_provider_connection(
    Path(provider): Path<String>,
    Json(request): Json<ProviderConnectionTestPayload>,
) -> ApiResult<HashMap<String, String>> {
    // Validate input
    request.validate().map_err(|e| {
        AyiahError::ApiError(ApiError::BadRequest(format!("Validation error: {}", e)))
    })?;

    // TODO: Implement provider connection test
    let mut result = HashMap::new();
    result.insert("provider".to_string(), provider);
    result.insert("status".to_string(), "connected".to_string());
    result.insert("response_time".to_string(), "150ms".to_string());

    Ok(ApiResponse {
        code: StatusCode::OK.into(),
        message: "Provider test completed".to_string(),
        data: Some(result),
    })
}
