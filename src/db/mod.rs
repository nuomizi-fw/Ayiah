use crate::app::config::ConfigManager;
use crate::error::AyiahError;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::time::Duration;

pub type Database = Pool<Sqlite>;

pub async fn init() -> Result<Database, AyiahError> {
    let db_config = {
        let config = ConfigManager::instance()
            .expect("Configuration not initialized")
            .read();
        config.database.clone()
    };

    // 只支持 SQLite
    let db_path = if db_config.db_file.is_empty() {
        "ayiah.db"
    } else {
        &db_config.db_file
    };

    let pool = SqlitePool::connect_with(
        sqlx::sqlite::SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .busy_timeout(Duration::from_secs(30)),
    )
    .await
    .map_err(|e| AyiahError::DatabaseError(e.to_string()))?;

    // 运行迁移
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| AyiahError::DatabaseError(format!("Migration failed: {}", e)))?;

    Ok(pool)
}
