use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use crate::{
    ApiResponse, ApiResult, Ctx,
    entities::{CreateLibraryFolder, LibraryFolder},
    services::{FileScanner, ScanResult},
};

/// Create library folder request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLibraryFolderRequest {
    pub name: String,
    pub path: String,
    pub media_type: crate::entities::MediaType,
}

/// Scan response
#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResponse {
    pub folder: LibraryFolder,
    pub result: ScanResult,
}

/// List all library folders
async fn list_folders(State(ctx): State<Ctx>) -> ApiResult<Vec<LibraryFolder>> {
    let folders = LibraryFolder::list_all(&ctx.db).await.map_err(|e| {
        crate::error::AyiahError::DatabaseError(format!("Failed to fetch library folders: {e}"))
    })?;

    Ok(ApiResponse {
        code: 200,
        message: "Library folders retrieved successfully".to_string(),
        data: Some(folders),
    })
}

/// Get library folder by ID
async fn get_folder(State(ctx): State<Ctx>, Path(id): Path<i64>) -> ApiResult<LibraryFolder> {
    let folder = LibraryFolder::find_by_id(&ctx.db, id)
        .await
        .map_err(|e| {
            crate::error::AyiahError::DatabaseError(format!("Failed to fetch library folder: {e}"))
        })?
        .ok_or_else(|| {
            crate::error::AyiahError::ApiError(crate::error::ApiError::NotFound(format!(
                "Library folder with ID {id} not found"
            )))
        })?;

    Ok(ApiResponse {
        code: 200,
        message: "Library folder retrieved successfully".to_string(),
        data: Some(folder),
    })
}

/// Create a new library folder
async fn create_folder(
    State(ctx): State<Ctx>,
    Json(request): Json<CreateLibraryFolderRequest>,
) -> ApiResult<LibraryFolder> {
    // Validate path exists
    let path = std::path::Path::new(&request.path);
    if !path.exists() {
        return Err(crate::error::AyiahError::ApiError(
            crate::error::ApiError::BadRequest(format!("Path does not exist: {}", request.path)),
        ));
    }

    if !path.is_dir() {
        return Err(crate::error::AyiahError::ApiError(
            crate::error::ApiError::BadRequest(format!(
                "Path is not a directory: {}",
                request.path
            )),
        ));
    }

    let create_folder = CreateLibraryFolder {
        name: request.name,
        path: request.path,
        media_type: request.media_type,
    };

    let folder = LibraryFolder::create(&ctx.db, create_folder)
        .await
        .map_err(|e| {
            crate::error::AyiahError::DatabaseError(format!("Failed to create library folder: {e}"))
        })?;

    Ok(ApiResponse {
        code: 201,
        message: "Library folder created successfully".to_string(),
        data: Some(folder),
    })
}

/// Delete a library folder
async fn delete_folder(
    State(ctx): State<Ctx>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, Json<ApiResponse<String>>)> {
    LibraryFolder::delete(&ctx.db, id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                code: 500,
                message: format!("Failed to delete library folder: {e}"),
                data: None,
            }),
        )
    })?;

    Ok(Json(ApiResponse {
        code: 200,
        message: "Library folder deleted successfully".to_string(),
        data: Some("Deleted".to_string()),
    }))
}

/// Scan a specific library folder
async fn scan_folder(
    State(ctx): State<Ctx>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<ScanResponse>>, (StatusCode, Json<ApiResponse<String>>)> {
    let folder = LibraryFolder::find_by_id(&ctx.db, id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    code: 500,
                    message: format!("Failed to fetch library folder: {e}"),
                    data: None,
                }),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse {
                    code: 404,
                    message: format!("Library folder with ID {id} not found"),
                    data: None,
                }),
            )
        })?;

    let scanner = FileScanner::new(ctx.db.clone());
    let result = scanner.scan_library_folder(&folder).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                code: 500,
                message: format!("Failed to scan library folder: {e}"),
                data: None,
            }),
        )
    })?;

    // If metadata agent is available, fetch metadata for new items
    if let Some(metadata_agent) = &ctx.metadata_agent {
        tokio::spawn({
            let metadata_agent = metadata_agent.clone();
            let db = ctx.db.clone();
            let folder_id = folder.id;
            async move {
                // Get all media items without metadata from this folder
                let items = match sqlx::query_as::<_, crate::entities::MediaItem>(
                    "SELECT * FROM media_items WHERE library_folder_id = ? AND id NOT IN (SELECT media_item_id FROM video_metadata)"
                )
                .bind(folder_id)
                .fetch_all(&db)
                .await {
                    Ok(items) => items,
                    Err(e) => {
                        tracing::error!("Failed to fetch items without metadata: {}", e);
                        return;
                    }
                };

                tracing::info!("Fetching metadata for {} items", items.len());
                let results = metadata_agent.batch_fetch_metadata(items).await;

                let success_count = results.iter().filter(|r| r.is_ok()).count();
                tracing::info!(
                    "Metadata fetch complete: {}/{} successful",
                    success_count,
                    results.len()
                );
            }
        });
    }

    Ok(Json(ApiResponse {
        code: 200,
        message: "Library folder scanned successfully".to_string(),
        data: Some(ScanResponse { folder, result }),
    }))
}

/// Scan all library folders
async fn scan_all_folders(
    State(ctx): State<Ctx>,
) -> Result<Json<ApiResponse<Vec<ScanResponse>>>, (StatusCode, Json<ApiResponse<String>>)> {
    let scanner = FileScanner::new(ctx.db.clone());
    let results = scanner.scan_all_libraries().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                code: 500,
                message: format!("Failed to scan libraries: {e}"),
                data: None,
            }),
        )
    })?;

    let response: Vec<ScanResponse> = results
        .into_iter()
        .map(|(folder, result)| ScanResponse { folder, result })
        .collect();

    Ok(Json(ApiResponse {
        code: 200,
        message: "All libraries scanned successfully".to_string(),
        data: Some(response),
    }))
}

/// Mount library folder routes
pub fn mount() -> Router<Ctx> {
    Router::new()
        .route("/library-folders", get(list_folders).post(create_folder))
        .route(
            "/library-folders/{id}",
            get(get_folder).delete(delete_folder),
        )
        .route("/library-folders/{id}/scan", post(scan_folder))
        .route("/library-folders/scan-all", post(scan_all_folders))
}
