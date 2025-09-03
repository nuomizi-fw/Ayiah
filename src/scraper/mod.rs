pub mod provider;

use provider::{MediaMetadata, MetadataProvider};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};
use tokio::{
    sync::{Semaphore, mpsc},
    task::JoinSet,
};
use walkdir::WalkDir;

use crate::error::ScrapeError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrganizeMethod {
    SoftLink,
    HardLink,
    Copy,
    Move,
}

impl Default for OrganizeMethod {
    fn default() -> Self {
        Self::SoftLink
    }
}

/// Media type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MediaType {
    /// Video file
    Video,
    /// Book file
    Book,
    /// Music file
    Music,
    /// Comic file
    Comic,
}

/// Scraper provider enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Provider {
    /// TMDB (The Movie Database)
    Tmdb,
    /// TVDB (The TV Database)
    Tvdb,
    /// Anilist
    Anilist,
    /// Bangumi
    Bangumi,
}

/// File information for processing
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub media_type: MediaType,
    pub size: u64,
    pub modified: std::time::SystemTime,
}

/// Scraping progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapeProgress {
    pub total_files: usize,
    pub processed_files: usize,
    pub successful_files: usize,
    pub failed_files: usize,
    pub current_file: Option<String>,
    pub elapsed_time: u64, // in seconds
}

/// Default file extensions for each media type
fn get_default_extensions() -> HashMap<MediaType, Vec<String>> {
    let mut extensions = HashMap::new();

    extensions.insert(
        MediaType::Video,
        vec![
            "mp4".to_string(),
            "mkv".to_string(),
            "avi".to_string(),
            "mov".to_string(),
            "wmv".to_string(),
            "flv".to_string(),
            "webm".to_string(),
            "m4v".to_string(),
            "mpg".to_string(),
            "mpeg".to_string(),
            "3gp".to_string(),
            "ts".to_string(),
        ],
    );

    extensions.insert(
        MediaType::Book,
        vec![
            "epub".to_string(),
            "mobi".to_string(),
            "pdf".to_string(),
            "azw".to_string(),
            "azw3".to_string(),
            "txt".to_string(),
            "fb2".to_string(),
            "lit".to_string(),
            "pdb".to_string(),
        ],
    );

    extensions.insert(
        MediaType::Music,
        vec![
            "mp3".to_string(),
            "flac".to_string(),
            "wav".to_string(),
            "aac".to_string(),
            "ogg".to_string(),
            "m4a".to_string(),
            "wma".to_string(),
            "opus".to_string(),
            "aiff".to_string(),
        ],
    );

    extensions.insert(
        MediaType::Comic,
        vec![
            "cbz".to_string(),
            "cbr".to_string(),
            "cb7".to_string(),
            "cbt".to_string(),
            "pdf".to_string(),
        ],
    );

    extensions
}

/// Main struct for Ayiah Scraper
pub struct AyiahScraper {
    pub organize_method: OrganizeMethod,
    pub source_path: PathBuf,
    pub target_path: PathBuf,
    pub metadata_provider: Arc<dyn MetadataProvider>,
    pub max_concurrent_tasks: usize,
    pub chunk_size: usize,
    pub retry_count: usize,
    pub skip_existing: bool,
    pub dry_run: bool,
}

impl AyiahScraper {
    pub fn new(
        organize_method: OrganizeMethod,
        source_path: PathBuf,
        target_path: PathBuf,
        metadata_provider: Arc<dyn MetadataProvider>,
    ) -> Self {
        Self {
            organize_method,
            source_path,
            target_path,
            metadata_provider,
            max_concurrent_tasks: num_cpus::get() * 2,
            chunk_size: 100,
            retry_count: 3,
            skip_existing: true,
            dry_run: false,
        }
    }

    pub fn with_concurrency(mut self, max_concurrent_tasks: usize) -> Self {
        self.max_concurrent_tasks = max_concurrent_tasks;
        self
    }

    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size;
        self
    }

    pub fn with_retry_count(mut self, retry_count: usize) -> Self {
        self.retry_count = retry_count;
        self
    }

    pub fn with_skip_existing(mut self, skip_existing: bool) -> Self {
        self.skip_existing = skip_existing;
        self
    }

    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Efficiently scan media files in source directory
    pub fn scan_files(&self) -> Result<Vec<FileInfo>, ScrapeError> {
        let start_time = Instant::now();
        tracing::info!("Starting directory scan: {:?}", self.source_path);

        if !self.source_path.exists() {
            return Err(ScrapeError::FileNotFound(format!(
                "Source directory does not exist: {:?}",
                self.source_path
            )));
        }

        // Use rayon for parallel file scanning
        let files: Vec<FileInfo> = WalkDir::new(&self.source_path)
            .into_iter()
            .par_bridge()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();

                // Only process files, skip directories
                if !path.is_file() {
                    return None;
                }

                // Check file extension
                let extension = path.extension()?.to_str()?.to_lowercase();
                let media_type = self.detect_media_type(&extension)?;

                // Get file metadata
                let metadata = fs::metadata(path).ok()?;
                let size = metadata.len();
                let modified = metadata.modified().ok()?;

                Some(FileInfo {
                    path: path.to_path_buf(),
                    media_type,
                    size,
                    modified,
                })
            })
            .collect();
        let elapsed = start_time.elapsed();

        tracing::info!(
            "Scan completed: found {} media files, took {:?}",
            files.len(),
            elapsed
        );

        Ok(files)
    }

    /// Detect media type from file extension
    pub fn detect_media_type(&self, extension: &str) -> Option<MediaType> {
        let extensions = get_default_extensions();
        for (media_type, exts) in &extensions {
            if exts.contains(&extension.to_string()) {
                return Some(media_type.clone());
            }
        }
        None
    }

    /// Main scraping method - efficient concurrent processing
    pub async fn run(&self) -> Result<ScrapeProgress, ScrapeError> {
        let start_time = Instant::now();
        let files = self.scan_files()?;

        if files.is_empty() {
            tracing::warn!("No media files found");
            return Ok(ScrapeProgress {
                total_files: 0,
                processed_files: 0,
                successful_files: 0,
                failed_files: 0,
                current_file: None,
                elapsed_time: start_time.elapsed().as_secs(),
            });
        }

        let total_files = files.len();
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent_tasks));
        let (progress_tx, mut progress_rx) = mpsc::channel(1000);

        // 创建进度跟踪任务
        let progress_handle = {
            let total = total_files;
            tokio::spawn(async move {
                let mut progress = ScrapeProgress {
                    total_files: total,
                    processed_files: 0,
                    successful_files: 0,
                    failed_files: 0,
                    current_file: None,
                    elapsed_time: 0,
                };

                while let Some(update) = progress_rx.recv().await {
                    match update {
                        ProgressUpdate::FileStarted(file_path) => {
                            progress.current_file = Some(file_path);
                        }
                        ProgressUpdate::FileCompleted(success) => {
                            progress.processed_files += 1;
                            if success {
                                progress.successful_files += 1;
                            } else {
                                progress.failed_files += 1;
                            }
                            progress.elapsed_time = start_time.elapsed().as_secs();

                            // Every 10 files print progress
                            if progress.processed_files % 10 == 0 {
                                tracing::info!(
                                    "Progress: {}/{} ({:.1}%) - Success: {}, Failed: {}",
                                    progress.processed_files,
                                    progress.total_files,
                                    (progress.processed_files as f32 / progress.total_files as f32)
                                        * 100.0,
                                    progress.successful_files,
                                    progress.failed_files
                                );
                            }
                        }
                    }
                }

                progress
            })
        };

        // Process files in chunks to avoid excessive memory usage
        let chunks: Vec<_> = files.chunks(self.chunk_size).collect();
        let mut join_set = JoinSet::new();

        for chunk in chunks {
            let chunk_files = chunk.to_vec();
            let semaphore = Arc::clone(&semaphore);
            let progress_tx = progress_tx.clone();
            let scraper = self.clone_for_task();

            join_set.spawn(async move {
                scraper
                    .process_file_chunk(chunk_files, semaphore, progress_tx)
                    .await
            });
        }

        // Close the sender so the receiver knows when to stop
        drop(progress_tx);

        // Wait for all tasks to complete
        while let Some(result) = join_set.join_next().await {
            if let Err(e) = result {
                tracing::error!("Task execution failed: {:?}", e);
            }
        }

        // Get final progress
        let final_progress = progress_handle.await?;

        tracing::info!(
            "Scraping completed! Total: {}, Success: {}, Failed: {}, Time: {} seconds",
            final_progress.total_files,
            final_progress.successful_files,
            final_progress.failed_files,
            final_progress.elapsed_time
        );

        Ok(final_progress)
    }

    /// Clone necessary fields for async tasks
    fn clone_for_task(&self) -> AyiahScraper {
        AyiahScraper {
            organize_method: self.organize_method.clone(),
            source_path: self.source_path.clone(),
            target_path: self.target_path.clone(),
            metadata_provider: Arc::clone(&self.metadata_provider),
            max_concurrent_tasks: self.max_concurrent_tasks,
            chunk_size: self.chunk_size,
            retry_count: self.retry_count,
            skip_existing: self.skip_existing,
            dry_run: self.dry_run,
        }
    }

    /// Process file chunk
    async fn process_file_chunk(
        &self,
        files: Vec<FileInfo>,
        semaphore: Arc<Semaphore>,
        progress_tx: mpsc::Sender<ProgressUpdate>,
    ) -> Result<(), ScrapeError> {
        let mut join_set = JoinSet::new();

        for file_info in files {
            let semaphore = Arc::clone(&semaphore);
            let file_path = file_info.path.clone();
            let progress_tx = progress_tx.clone();
            let scraper = self.clone_for_task();

            join_set.spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();

                // Send start processing message
                let _ = progress_tx
                    .send(ProgressUpdate::FileStarted(
                        file_path.to_string_lossy().to_string(),
                    ))
                    .await;

                let success = scraper.process_single_file(&file_info).await.is_ok();

                // Send completion processing message
                let _ = progress_tx
                    .send(ProgressUpdate::FileCompleted(success))
                    .await;

                success
            });
        }

        // Wait for all files in the chunk to be processed
        while let Some(result) = join_set.join_next().await {
            if let Err(e) = result {
                tracing::error!("File processing task failed: {:?}", e);
            }
        }

        Ok(())
    }

    /// Process a single file
    pub async fn process_single_file(&self, file_info: &FileInfo) -> Result<(), ScrapeError> {
        let file_path = &file_info.path;

        tracing::debug!("Starting to process file: {:?}", file_path);

        // If in dry run mode, only simulate processing
        if self.dry_run {
            tracing::info!("[DRY RUN] Would process file: {:?}", file_path);
            return Ok(());
        }

        // Retry mechanism
        let mut last_error = None;
        for attempt in 1..=self.retry_count {
            match self.process_file_with_retry(file_info).await {
                Ok(_) => {
                    tracing::debug!("File processing successful: {:?}", file_path);
                    return Ok(());
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.retry_count {
                        tracing::warn!(
                            "File processing failed (attempt {}/{}): {:?} - error: {:?}",
                            attempt,
                            self.retry_count,
                            file_path,
                            last_error
                        );
                        tokio::time::sleep(tokio::time::Duration::from_millis(
                            1000 * attempt as u64,
                        ))
                        .await;
                    }
                }
            }
        }

        let error = last_error.unwrap();
        tracing::error!(
            "File processing finally failed: {:?} - error: {:?}",
            file_path,
            error
        );
        Err(error)
    }

    /// Process file with retry
    async fn process_file_with_retry(&self, file_info: &FileInfo) -> Result<(), ScrapeError> {
        // 1. Get metadata
        let metadata = self
            .metadata_provider
            .fetch_metadata()
            .await
            .map_err(|e| ScrapeError::MetadataFetchError(e.to_string()))?;

        // 2. Generate target path
        let target_path = self.generate_target_path(file_info, &metadata)?;

        // 3. Check if should skip existing files
        if self.skip_existing && target_path.exists() {
            tracing::debug!("Skipping existing file: {:?}", target_path);
            return Ok(());
        }

        // 4. Create target directory
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ScrapeError::DirectoryCreationError(e.to_string()))?;
        }

        // 5. Organize file
        self.organize_file(&file_info.path, &target_path).await?;

        tracing::info!(
            "File processing completed: {:?} -> {:?}",
            file_info.path,
            target_path
        );
        Ok(())
    }

    /// Generate target path
    fn generate_target_path(
        &self,
        file_info: &FileInfo,
        metadata: &MediaMetadata,
    ) -> Result<PathBuf, ScrapeError> {
        let mut target_path = self.target_path.clone();

        match metadata {
            MediaMetadata::Video(video) => {
                // Video file organization structure: target/Videos/Title (Year)/Season X/Title - SxxExx - Episode Title.ext
                target_path.push("Videos");

                let title = video.title.as_deref().unwrap_or("Unknown");
                let year = video
                    .release_date
                    .as_ref()
                    .and_then(|d| d.split('-').next())
                    .unwrap_or("Unknown");

                target_path.push(format!("{} ({})", title, year));

                if let Some(season) = video.season_number {
                    target_path.push(format!("Season {}", season));
                }
            }
            MediaMetadata::Book(book) => {
                // Book organization structure: target/Books/Author/Series/Title.ext
                target_path.push("Books");

                let author = book
                    .authors
                    .as_ref()
                    .and_then(|authors| authors.first())
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown Author");
                target_path.push(author);

                if let Some(series) = &book.series {
                    target_path.push(series);
                }
            }
            MediaMetadata::Music(music) => {
                // Music organization structure: target/Music/Artist/Album/Track - Title.ext
                target_path.push("Music");

                let artist = music
                    .artists
                    .as_ref()
                    .and_then(|artists| artists.first())
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown Artist");
                target_path.push(artist);

                if let Some(album) = &music.album {
                    target_path.push(album);
                }
            }
            MediaMetadata::Comic(comic) => {
                // Comic organization structure: target/Comics/Publisher/Series/Volume X/Issue.ext
                target_path.push("Comics");

                if let Some(publisher) = &comic.publisher {
                    target_path.push(publisher);
                }

                if let Some(series) = &comic.series {
                    target_path.push(series);

                    if let Some(volume) = comic.volume {
                        target_path.push(format!("Volume {}", volume));
                    }
                }
            }
        }

        // Add file name
        let filename = file_info
            .path
            .file_name()
            .ok_or_else(|| ScrapeError::InvalidPath("Invalid file path".to_string()))?;
        target_path.push(filename);

        Ok(target_path)
    }

    /// Organize file to target location
    pub async fn organize_file(&self, source: &Path, target: &Path) -> Result<(), ScrapeError> {
        match self.organize_method {
            OrganizeMethod::SoftLink => {
                #[cfg(unix)]
                {
                    std::os::unix::fs::symlink(source, target)
                        .map_err(|e| ScrapeError::SymlinkError(e.to_string()))?;
                }
                #[cfg(windows)]
                {
                    if source.is_file() {
                        std::os::windows::fs::symlink_file(source, target)
                            .map_err(|e| ScrapeError::SymlinkError(e.to_string()))?;
                    } else {
                        std::os::windows::fs::symlink_dir(source, target)
                            .map_err(|e| ScrapeError::SymlinkError(e.to_string()))?;
                    }
                }
            }
            OrganizeMethod::HardLink => {
                fs::hard_link(source, target)
                    .map_err(|e| ScrapeError::HardLinkError(e.to_string()))?;
            }
            OrganizeMethod::Copy => {
                tokio::fs::copy(source, target)
                    .await
                    .map_err(|e| ScrapeError::CopyError(e.to_string()))?;
            }
            OrganizeMethod::Move => {
                tokio::fs::rename(source, target)
                    .await
                    .map_err(|e| ScrapeError::MoveError(e.to_string()))?;
            }
        }
        Ok(())
    }

    /// Modify existing organization structure
    pub async fn modify(&self) -> Result<(), ScrapeError> {
        // Here you can implement the logic to reorganize existing files
        tracing::info!("Start reorganization of existing files...");
        // TODO: Implement reorganization logic
        Ok(())
    }
}

/// Progress update message
#[derive(Debug)]
enum ProgressUpdate {
    FileStarted(String),
    FileCompleted(bool),
}
