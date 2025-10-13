use crate::error::AyiahError;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::path::PathBuf;
use std::time::Duration;

pub type Database = Pool<Sqlite>;

/// Get database file path following XDG Base Directory specification
/// or `AYIAH_DATA_DIR` environment variable for Docker deployment
fn get_db_path() -> PathBuf {
    std::env::var("AYIAH_DATA_DIR").map_or_else(
        |_| {
            dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("ayiah")
                .join("ayiah.db")
        },
        |data_dir| PathBuf::from(data_dir).join("ayiah.db"),
    )
}

pub async fn init() -> Result<Database, AyiahError> {
    let db_path = get_db_path();

    // Ensure the parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            AyiahError::DatabaseError(format!("Failed to create database directory: {e}"))
        })?;
    }

    let pool = SqlitePool::connect_with(
        sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .busy_timeout(Duration::from_secs(30)),
    )
    .await
    .map_err(|e| AyiahError::DatabaseError(e.to_string()))?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| AyiahError::DatabaseError(format!("Migration failed: {e}")))?;

    Ok(pool)
}
