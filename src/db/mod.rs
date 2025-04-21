pub mod entity;
pub mod migration;

use crate::error::AyiahError;

use crate::app::config::ConfigManager;
use sea_orm::{Database, DatabaseConnection};

pub async fn init() -> Result<DatabaseConnection, AyiahError> {
    let db_config = {
        let config = ConfigManager::instance()
            .expect("Configuration not initialized")
            .read();
        config.database.clone()
    };

    let conn_str = match db_config.db_type.as_str() {
        "sqlite" => {
            let db_path = if db_config.db_file.is_empty() {
                "ayiah.db"
            } else {
                &db_config.db_file
            };
            format!("sqlite:{}?mode=rwc", db_path)
        }
        "postgres" => {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                db_config.user, db_config.password, db_config.host, db_config.port, db_config.name
            )
        }
        "mysql" => {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                db_config.user, db_config.password, db_config.host, db_config.port, db_config.name
            )
        }
        db_type => {
            return Err(AyiahError::DbError(sea_orm::DbErr::Custom(format!(
                "Unsupported database type: {}",
                db_type
            ))));
        }
    };

    let mut opts = sea_orm::ConnectOptions::new(conn_str);

    // Common options for all database types
    opts.max_connections(20)
        .min_connections(5)
        .connect_timeout(std::time::Duration::from_secs(8))
        .sqlx_logging(true);

    Database::connect(opts).await.map_err(AyiahError::DbError)
}
