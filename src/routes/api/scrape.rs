use axum::{
    Router,
    extract::{Extension, Json, Query},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{
    ApiResponse, ApiResult, Ctx,
    app::config::ScrapeConfig,
    error::{ApiError, AyiahError},
    middleware::auth::JwtClaims,
    scraper::{MediaType, OrganizeMethod, Provider, provider},
};

/// Get configuration query parameters
#[derive(Debug, Deserialize, ToSchema)]
pub struct GetConfigQuery {
    /// Media type filter
    pub media_type: Option<MediaType>,
}

/// Unified scrape request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ScrapePayload {
    /// Scrape target type
    pub target: ScrapeTarget,
    /// Media type (optional, auto-detect)
    pub media_type: Option<MediaType>,
    /// Provider (optional, use default config)
    pub provider: Option<Provider>,
    /// Whether to auto organize files
    pub auto_organize: Option<bool>,
    /// Organize method
    pub organize_method: Option<OrganizeMethod>,
    /// Concurrency limit
    #[validate(range(
        min = 1,
        max = 10,
        message = "Concurrent limit must be between 1 and 10"
    ))]
    pub concurrent_limit: Option<u32>,
}

/// Scrape target type
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ScrapeTarget {
    /// Target type: file, batch, directory
    #[serde(rename = "type")]
    pub target_type: String,
    /// Single file path (used when type = "file")
    #[validate(length(min = 1, message = "File path cannot be empty"))]
    pub file_path: Option<String>,
    /// Batch file path list (used when type = "batch")
    #[validate(length(min = 1, max = 100, message = "File list must contain 1-100 files"))]
    pub file_paths: Option<Vec<String>>,
    /// Directory path (used when type = "directory")
    #[validate(length(min = 1, message = "Directory path cannot be empty"))]
    pub directory_path: Option<String>,
    /// Whether to recursively scan subdirectories (used when type = "directory")
    pub recursive: Option<bool>,
    /// File extension filter (used when type = "directory")
    pub file_extensions: Option<Vec<String>>,
}

/// Unified scrape response
#[derive(Debug, Serialize, ToSchema)]
pub struct ScrapeResponse {
    /// Scrape target information
    pub target_info: ScrapeTargetInfo,
    /// Total file count
    pub total_files: u32,
    /// Success count
    pub success_count: u32,
    /// Failed count
    pub failed_count: u32,
    /// Total duration (milliseconds)
    pub duration_ms: u64,
    /// Detailed results
    pub results: Vec<ScrapeResult>,
}

/// Scrape target information
#[derive(Debug, Serialize, ToSchema)]
pub struct ScrapeTargetInfo {
    /// Target type: file, batch, directory
    #[serde(rename = "type")]
    pub target_type: String,
    /// Single file path (used when type = "file")
    pub file_path: Option<String>,
    /// Batch file processing count (used when type = "batch")
    pub processed_files: Option<u32>,
    /// Directory path (used when type = "directory")
    pub directory_path: Option<String>,
    /// Discovered file count (used when type = "directory")
    pub discovered_files: Option<u32>,
    /// Directory processed file count (used when type = "directory")
    pub directory_processed_files: Option<u32>,
}

/// Manual match request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ManualMatchPayload {
    /// File path
    #[validate(length(min = 1, message = "File path cannot be empty"))]
    pub file_path: String,
    /// Media type
    pub media_type: MediaType,
    /// Media ID (from provider)
    #[validate(length(min = 1, message = "Media ID cannot be empty"))]
    pub media_id: String,
    /// Provider
    pub provider: Provider,
    /// Whether to auto organize files
    pub auto_organize: Option<bool>,
    /// Organize method
    pub organize_method: Option<OrganizeMethod>,
}

/// Scrape result
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ScrapeResult {
    /// File path
    pub file_path: String,
    /// Media type
    pub media_type: MediaType,
    /// Used provider
    pub provider: Provider,
    /// Scrape status
    pub success: bool,
    /// Retrieved metadata
    pub metadata: Option<provider::MediaMetadata>,
    /// Organized file path
    pub organized_path: Option<String>,
    /// Processing time
    pub duration_ms: u64,
}

pub fn mount() -> Router {
    Router::new().nest(
        "/scrape",
        Router::new()
            // Unified scrape endpoint
            .route("/", post(scrape))
            // Other endpoints
            .route("/manual-match", post(manual_match))
            .route("/config", get(get_scrape_config).post(update_scrape_config)),
    )
}

/// Unified scrape endpoint
#[utoipa::path(
    post,
    operation_id = "scrape",
    path = "/api/scrape",
    tag = "Scraper",
    request_body = ScrapePayload,
    responses(
        (status = 200, description = "Scrape completed successfully", body = ApiResponse<ScrapeResponse>),
        (status = 400, description = "Invalid input data", body = ()),
        (status = 404, description = "File or directory not found", body = ()),
        (status = 500, description = "Internal server error", body = ()),
    ),
    params(),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn scrape(
    Extension(_ctx): Extension<Ctx>,
    _claims: JwtClaims,
    Json(payload): Json<ScrapePayload>,
) -> ApiResult<ScrapeResponse> {
    // Validate request parameters
    payload.validate().map_err(|e| {
        AyiahError::ApiError(ApiError::BadRequest(format!("Validation error: {}", e)))
    })?;

    Ok(ApiResponse {
        code: StatusCode::OK.into(),
        message: "Scrape completed successfully".to_string(),
        data: None,
    })
}

/// Manual match media information
#[utoipa::path(
    post,
    operation_id = "manual_match",
    path = "/api/scrape/manual-match",
    tag = "Scraper",
    request_body = ManualMatchPayload,
    responses(
        (status = 200, description = "Manual match completed", body = ApiResponse<ScrapeResult>),
        (status = 400, description = "Invalid input data", body = ()),
        (status = 404, description = "File or media not found", body = ()),
        (status = 500, description = "Internal server error", body = ()),
    ),
    params(),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn manual_match(
    Extension(_ctx): Extension<Ctx>,
    _claims: JwtClaims,
    Json(payload): Json<ManualMatchPayload>,
) -> ApiResult<ScrapeResult> {
    // Validate input
    payload.validate().map_err(|e| {
        AyiahError::ApiError(ApiError::BadRequest(format!("Validation error: {}", e)))
    })?;

    Ok(ApiResponse {
        code: StatusCode::OK.into(),
        message: "Manual match completed".to_string(),
        data: None,
    })
}

/// Get scraping configuration
#[utoipa::path(
    get,
    operation_id = "get_scrape_config",
    path = "/api/scrape/config",
    tag = "Scraper",
    responses(
        (status = 200, description = "Configuration retrieved", body = ApiResponse<ScrapeConfig>),
        (status = 500, description = "Internal server error", body = ()),
    ),
    params(
        ("media_type" = Option<MediaType>, Query, description = "Filter by media type")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_scrape_config(
    Extension(ctx): Extension<Ctx>,
    _claims: JwtClaims,
    Query(_query): Query<GetConfigQuery>,
) -> ApiResult<ScrapeConfig> {
    let config = ctx.config.read().scrape.clone();

    Ok(ApiResponse {
        code: StatusCode::OK.into(),
        message: "Configuration retrieved".to_string(),
        data: Some(config),
    })
}

/// Update scraping configuration
#[utoipa::path(
    post,
    operation_id = "update_scrape_config",
    path = "/api/scrape/config",
    tag = "Scraper",
    request_body = ScrapeConfig,
    responses(
        (status = 200, description = "Configuration updated", body = ()),
        (status = 400, description = "Invalid configuration", body = ()),
        (status = 500, description = "Internal server error", body = ()),
    ),
    params(),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_scrape_config(
    Extension(ctx): Extension<Ctx>,
    _claims: JwtClaims,
    Json(config): Json<ScrapeConfig>,
) -> ApiResult<()> {
    let mut app_config = ctx.config.write();
    app_config.scrape = config;

    Ok(ApiResponse {
        code: StatusCode::OK.into(),
        message: "Configuration updated".to_string(),
        data: None,
    })
}
