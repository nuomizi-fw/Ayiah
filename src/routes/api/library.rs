use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};

use crate::{
    ApiResponse, ApiResult, Ctx,
    entities::{MediaItemWithMetadata, MediaType},
};

/// Library API response
#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryResponse {
    pub items: Vec<MediaItemWithMetadata>,
    pub total: usize,
}

/// Get movies
async fn get_movies(State(ctx): State<Ctx>) -> ApiResult<LibraryResponse> {
    let items = MediaItemWithMetadata::list_by_type(&ctx.db, MediaType::Movie)
        .await
        .map_err(|e| {
            crate::error::AyiahError::DatabaseError(format!("Failed to fetch movies: {e}"))
        })?;

    let total = items.len();

    Ok(ApiResponse {
        code: 200,
        message: "Movies retrieved successfully".to_string(),
        data: Some(LibraryResponse { items, total }),
    })
}

/// Get TV shows
async fn get_tv_shows(State(ctx): State<Ctx>) -> ApiResult<LibraryResponse> {
    let items = MediaItemWithMetadata::list_by_type(&ctx.db, MediaType::Tv)
        .await
        .map_err(|e| {
            crate::error::AyiahError::DatabaseError(format!("Failed to fetch TV shows: {e}"))
        })?;

    let total = items.len();

    Ok(ApiResponse {
        code: 200,
        message: "TV shows retrieved successfully".to_string(),
        data: Some(LibraryResponse { items, total }),
    })
}

/// Get media item by ID
async fn get_media_item(
    State(ctx): State<Ctx>,
    Path(id): Path<i64>,
) -> ApiResult<MediaItemWithMetadata> {
    let item = MediaItemWithMetadata::find_by_id(&ctx.db, id)
        .await
        .map_err(|e| {
            crate::error::AyiahError::DatabaseError(format!("Failed to fetch media item: {e}"))
        })?
        .ok_or_else(|| {
            crate::error::AyiahError::ApiError(crate::error::ApiError::NotFound(format!(
                "Media item with ID {id} not found"
            )))
        })?;

    Ok(ApiResponse {
        code: 200,
        message: "Media item retrieved successfully".to_string(),
        data: Some(item),
    })
}

/// Refresh metadata for a media item
async fn refresh_metadata(
    State(ctx): State<Ctx>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, Json<ApiResponse<String>>)> {
    let metadata_agent = ctx.metadata_agent.as_ref().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ApiResponse {
                code: 503,
                message: "Metadata agent not available".to_string(),
                data: None,
            }),
        )
    })?;

    match metadata_agent.refresh_metadata(id).await {
        Ok(_) => Ok(Json(ApiResponse {
            code: 200,
            message: "Metadata refreshed successfully".to_string(),
            data: Some("Metadata updated".to_string()),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                code: 500,
                message: format!("Failed to refresh metadata: {e}"),
                data: None,
            }),
        )),
    }
}

/// Mount library routes
pub fn mount() -> Router<Ctx> {
    Router::new()
        .route("/library/movies", get(get_movies))
        .route("/library/tv", get(get_tv_shows))
        .route("/library/items/{id}", get(get_media_item))
        .route("/library/items/{id}/refresh", get(refresh_metadata))
}
