use std::sync::Arc;

use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;
use tokio::sync::RwLock;

use crate::migration::Migrator;

use super::config::ConfigManager;

/// AppState holds all shared application resources
#[derive(Clone)]
pub struct AppState {
    /// Shared configuration manager
    pub config: Arc<ConfigManager>,

    /// Database connection
    pub db: Arc<DatabaseConnection>,

    /// Any shared data that needs read/write access with locking
    pub shared_data: Arc<RwLock<SharedData>>,
}

/// Data shared across the application that requires read/write access
#[derive(Default)]
pub struct SharedData {}

/// Allow extracting the database connection directly from AppState
impl FromRef<AppState> for Arc<DatabaseConnection> {
    fn from_ref(state: &AppState) -> Arc<DatabaseConnection> {
        state.db.clone()
    }
}

/// Allow extracting the config manager directly from AppState
impl FromRef<AppState> for Arc<ConfigManager> {
    fn from_ref(state: &AppState) -> Arc<ConfigManager> {
        state.config.clone()
    }
}

impl AppState {
    /// Create a new instance of the application state
    pub async fn new(config: ConfigManager, db: DatabaseConnection) -> Self {
        Self {
            config: Arc::new(config),
            db: Arc::new(db),
            shared_data: Arc::new(RwLock::new(SharedData::default())),
        }
    }

    /// Initialize the application state from configuration
    pub async fn init(config: ConfigManager) -> Result<Self, anyhow::Error> {
        // Example: establish database connection using config
        let db_connection = establish_database_connection(&config).await?;

        Ok(Self::new(config, db_connection).await)
    }
}

/// Establish database connection using configuration
async fn establish_database_connection(
    config: &ConfigManager,
) -> Result<DatabaseConnection, anyhow::Error> {
    // Read database configuration
    let db_url = {
        let config = config.read();
        config.database.url.clone()
    };

    // Connect to database
    let conn = sea_orm::Database::connect(&db_url).await?;

    // Migrate database
    Migrator::up(&conn, None).await.unwrap();

    Ok(conn)
}
