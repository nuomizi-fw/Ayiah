use std::collections::HashMap;

use axum::{
    Extension, Json, Router,
    extract::Path,
    routing::{get, post},
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{
    ApiResponse, ApiResult, Ctx,
    error::{ApiError, AyiahError},
    middleware::auth::JwtClaims,
    scraper::{MediaType, Provider},
};

#[derive(Debug, Deserialize, Validate, ToSchema)]
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
#[derive(Debug, Serialize, ToSchema)]
pub struct ProvidersResponse {
    /// List of available providers
    pub providers: Vec<ProviderInfo>,
}

/// Provider information
#[derive(Debug, Serialize, ToSchema)]
pub struct ProviderInfo {
    /// Provider identifier
    pub id: Provider,
    /// Display name
    pub name: String,
    /// Description
    pub description: String,
    /// Supported media types
    pub supported_media_types: Vec<MediaType>,
    /// Whether API key is required
    pub requires_api_key: bool,
    /// Whether available
    pub available: bool,
}

pub fn mount() -> Router {
    Router::new().nest(
        "/provider",
        Router::new() // Get supported providers list
            .route("/providers", get(get_supported_providers))
            // Test provider connection
            .route(
                "/providers/{:provider}/test",
                post(test_provider_connection),
            ),
    )
}

/// Get supported providers list
#[utoipa::path(
    get,
    operation_id = "get_supported_providers",
    path = "/api/scrape/providers",
    tag = "Scraper",
    responses(
        (status = 200, description = "Providers list retrieved", body = ApiResponse<ProvidersResponse>),
        (status = 500, description = "Internal server error", body = ()),
    ),
    params(),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_supported_providers(
    Extension(_ctx): Extension<Ctx>,
    _claims: JwtClaims,
) -> ApiResult<ProvidersResponse> {
    // TODO: Detect availability of each provider
    let response = ProvidersResponse { providers: vec![] };

    Ok(ApiResponse {
        code: StatusCode::OK.into(),
        message: "Providers list retrieved".to_string(),
        data: Some(response),
    })
}

/// Test provider connection
#[utoipa::path(
    post,
    operation_id = "test_provider_connection",
    path = "/api/scrape/providers/{provider}/test",
    tag = "Scraper",
    request_body = ProviderConnectionTestPayload,
    responses(
        (status = 200, description = "Provider test completed", body = ApiResponse<HashMap<String, String>>),
        (status = 400, description = "Invalid provider or request", body = ()),
        (status = 500, description = "Internal server error", body = ()),
    ),
    params(
        ("provider" = String, Path, description = "Provider name to test")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn test_provider_connection(
    Extension(_ctx): Extension<Ctx>,
    _claims: JwtClaims,
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
