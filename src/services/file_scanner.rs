use crate::entities::{CreateMediaItem, LibraryFolder, MediaItem, MediaType};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, error, info, warn};
use walkdir::WalkDir;

/// File scanner service for detecting media files
pub struct FileScanner {
    db: sqlx::SqlitePool,
}

/// Scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub total_files: usize,
    pub new_items: usize,
    pub existing_items: usize,
    pub errors: usize,
}

impl FileScanner {
    /// Create a new file scanner
    pub fn new(db: sqlx::SqlitePool) -> Self {
        Self { db }
    }

    /// Scan a library folder for media files
    pub async fn scan_library_folder(
        &self,
        folder: &LibraryFolder,
    ) -> Result<ScanResult, FileScannerError> {
        info!("Scanning library folder: {} ({})", folder.name, folder.path);

        let path = Path::new(&folder.path);
        if !path.exists() {
            return Err(FileScannerError::PathNotFound(folder.path.clone()));
        }

        if !path.is_dir() {
            return Err(FileScannerError::NotADirectory(folder.path.clone()));
        }

        let mut total_files = 0;
        let mut new_items = 0;
        let mut existing_items = 0;
        let mut errors = 0;

        // Get supported extensions for this media type
        let extensions = get_supported_extensions(folder.media_type);

        // Walk through directory
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();

            // Skip directories
            if entry_path.is_dir() {
                continue;
            }

            // Check if file has supported extension
            if let Some(ext) = entry_path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if !extensions.contains(&ext_str.as_str()) {
                    continue;
                }
            } else {
                continue;
            }

            total_files += 1;

            // Get file metadata
            let file_path = entry_path.to_string_lossy().to_string();
            let file_size = match entry.metadata() {
                Ok(metadata) => metadata.len() as i64,
                Err(e) => {
                    error!("Failed to get metadata for {}: {}", file_path, e);
                    errors += 1;
                    continue;
                }
            };

            // Extract title from filename
            let title = extract_title(entry_path);

            // Check if item already exists
            match MediaItem::find_by_path(&self.db, &file_path).await {
                Ok(Some(_)) => {
                    debug!("Media item already exists: {}", file_path);
                    existing_items += 1;
                }
                Ok(None) => {
                    // Create new media item
                    let create_item = CreateMediaItem {
                        library_folder_id: folder.id,
                        media_type: folder.media_type,
                        title: title.clone(),
                        file_path: file_path.clone(),
                        file_size,
                    };

                    match MediaItem::create(&self.db, create_item).await {
                        Ok(_) => {
                            info!("Added new media item: {}", title);
                            new_items += 1;
                        }
                        Err(e) => {
                            error!("Failed to create media item for {}: {}", file_path, e);
                            errors += 1;
                        }
                    }
                }
                Err(e) => {
                    error!("Database error while checking {}: {}", file_path, e);
                    errors += 1;
                }
            }
        }

        info!(
            "Scan complete: {} total files, {} new, {} existing, {} errors",
            total_files, new_items, existing_items, errors
        );

        Ok(ScanResult {
            total_files,
            new_items,
            existing_items,
            errors,
        })
    }

    /// Scan all enabled library folders
    pub async fn scan_all_libraries(
        &self,
    ) -> Result<Vec<(LibraryFolder, ScanResult)>, FileScannerError> {
        let folders = LibraryFolder::list_enabled(&self.db)
            .await
            .map_err(|e| FileScannerError::DatabaseError(e.to_string()))?;

        let mut results = Vec::new();

        for folder in folders {
            match self.scan_library_folder(&folder).await {
                Ok(result) => {
                    results.push((folder, result));
                }
                Err(e) => {
                    warn!("Failed to scan folder {}: {}", folder.name, e);
                    results.push((
                        folder,
                        ScanResult {
                            total_files: 0,
                            new_items: 0,
                            existing_items: 0,
                            errors: 1,
                        },
                    ));
                }
            }
        }

        Ok(results)
    }
}

/// Get supported file extensions for a media type
fn get_supported_extensions(media_type: MediaType) -> Vec<&'static str> {
    match media_type {
        MediaType::Movie | MediaType::Tv => vec![
            "mkv", "mp4", "avi", "mov", "wmv", "flv", "webm", "m4v", "mpg", "mpeg", "m2ts", "ts",
        ],
        MediaType::Comic => vec!["cbz", "cbr", "cb7", "cbt", "pdf"],
        MediaType::Book => vec!["epub", "mobi", "azw3", "pdf"],
    }
}

/// Extract title from file path
fn extract_title(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown")
        .to_string()
}

/// File scanner errors
#[derive(Debug, thiserror::Error)]
pub enum FileScannerError {
    #[error("Path not found: {0}")]
    PathNotFound(String),

    #[error("Not a directory: {0}")]
    NotADirectory(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
