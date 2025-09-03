use std::{
    fs,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
};

use config::{Config as ConfigBuilder, Environment, File as ConfigFile};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    error::ConfigError,
    scraper::{
        OrganizeMethod, Provider,
        provider::{
            anilist::AnilistProvider, bangumi::BangumiProvider, tmdb::TmdbProvider,
            tvdb::TvdbProvider,
        },
    },
};

// Global configuration manager instance
static CONFIG_MANAGER: OnceCell<ConfigManager> = OnceCell::new();

// Default configuration path
const DEFAULT_CONFIG_PATH: &str = "config/ayiah.toml";
const ENVIRONMENT_PREFIX: &str = "AYIAH";

/// Configuration manager
#[derive(Debug, Clone)]
pub struct ConfigManager {
    config: Arc<RwLock<AppConfig>>,
    config_path: PathBuf,
}

// Application configuration structure
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub server: ServerConfig,

    #[serde(default)]
    pub database: DatabaseConfig,

    #[serde(default)]
    pub auth: AuthConfig,

    #[serde(default)]
    pub logging: LoggingConfig,

    #[serde(default)]
    pub providers: ProvidersConfig,

    #[serde(default)]
    pub scrape: ScrapeConfig,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ProvidersConfig {
    #[serde(default)]
    pub tmdb: TmdbProvider,

    #[serde(default)]
    pub tvdb: TvdbProvider,

    #[serde(default)]
    pub anilist: AnilistProvider,

    #[serde(default)]
    pub bangumi: BangumiProvider,
}

/// Scraper configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapeConfig {
    /// Default provider
    pub default_provider: Provider,
    /// Fallback provider list
    pub fallback_providers: Vec<Provider>,
    /// Default organize method
    pub default_organize_method: OrganizeMethod,
}

impl Default for ScrapeConfig {
    fn default() -> Self {
        Self {
            default_provider: Provider::Tmdb,
            fallback_providers: vec![],
            default_organize_method: OrganizeMethod::HardLink,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default)]
    pub host: String,

    #[serde(default)]
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 7590,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    #[serde(default)]
    pub db_type: String,

    #[serde(default)]
    pub host: String,

    #[serde(default)]
    pub port: u16,

    #[serde(default)]
    pub user: String,

    #[serde(default)]
    pub password: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub db_file: String,

    #[serde(default)]
    pub table_prefix: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            db_type: "sqlite".to_string(),
            host: "".to_string(),
            port: 0,
            user: "".to_string(),
            password: "".to_string(),
            name: "".to_string(),
            db_file: "ayiah.db".to_string(),
            table_prefix: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    #[serde(default)]
    pub jwt_secret: String,

    #[serde(default)]
    pub jwt_expiry_hours: u64,

    #[serde(default)]
    pub pbkdf2_iterations: u32,

    #[serde(default)]
    pub refresh_token_expiry_days: u64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "ayiah".to_string(),
            jwt_expiry_hours: 24,
            pbkdf2_iterations: 100000,
            refresh_token_expiry_days: 7,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(default)]
    pub level: String,

    #[serde(default)]
    pub file_path: Option<String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file_path: None,
        }
    }
}

impl ConfigManager {
    /// Create a new configuration manager instance
    pub fn new<P: AsRef<Path>>(config_path: Option<P>) -> Result<Self, ConfigError> {
        let config_path = config_path
            .map(|p| p.as_ref().to_path_buf())
            .unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_PATH));

        let config = Self::load_config(&config_path)?;
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            config_path,
        })
    }

    /// Initialize the global configuration manager instance
    pub fn init<P: AsRef<Path>>(config_path: Option<P>) -> Result<&'static Self, ConfigError> {
        let config_path = config_path
            .map(|p| p.as_ref().to_path_buf())
            .unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_PATH));

        info!("Initializing configuration from {:?}", config_path);

        let manager = CONFIG_MANAGER.get_or_init(|| match Self::new(Some(&config_path)) {
            Ok(manager) => manager,
            Err(e) => {
                panic!("Failed to initialize configuration: {}", e);
            }
        });

        Ok(manager)
    }

    pub fn socket_addr(&self) -> Result<SocketAddr, ConfigError> {
        let config = self.config.read();
        let addr = format!("{}:{}", config.server.host, config.server.port)
            .parse::<SocketAddr>()
            .expect("Invalid server address configuration");
        Ok(addr)
    }

    /// Get the global configuration manager instance
    pub fn instance() -> Result<&'static Self, ConfigError> {
        CONFIG_MANAGER.get().ok_or(ConfigError::NotInitialized)
    }

    /// Get a read lock on the configuration
    pub fn read(&self) -> parking_lot::RwLockReadGuard<'_, AppConfig> {
        self.config.read()
    }

    /// Get a write lock on the configuration
    pub fn write(&self) -> parking_lot::RwLockWriteGuard<'_, AppConfig> {
        self.config.write()
    }

    /// Reload the configuration
    pub fn reload(&self) -> Result<(), ConfigError> {
        let new_config = Self::load_config(&self.config_path)?;
        let mut config = self.config.write();
        *config = new_config;
        info!("Configuration reloaded successfully");
        Ok(())
    }

    /// Reload the configuration from a specific path
    pub fn reload_from<P: AsRef<Path>>(&self, config_path: P) -> Result<(), ConfigError> {
        let new_config = Self::load_config(config_path)?;
        let mut config = self.config.write();
        *config = new_config;
        info!("Configuration reloaded successfully");
        Ok(())
    }

    /// Load configuration from file and environment variables
    fn load_config<P: AsRef<Path>>(config_path: P) -> Result<AppConfig, ConfigError> {
        let config_path = config_path.as_ref();

        // Check if the configuration file exists, if not, create default configuration
        if !config_path.exists() {
            info!(
                "Configuration file not found, creating default configuration at {:?}",
                config_path
            );
            if let Some(parent) = config_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).map_err(|e| {
                        ConfigError::WriteError(format!(
                            "Failed to create configuration directory: {}",
                            e
                        ))
                    })?;
                }
            }

            let default_config = AppConfig::default();
            let toml_str = toml::to_string_pretty(&default_config)
                .map_err(|e| ConfigError::ParseError(e.to_string()))?;

            fs::write(config_path, toml_str).map_err(|e| {
                ConfigError::WriteError(format!("Failed to write configuration file: {}", e))
            })?;
        }

        // Build configuration, combining file and environment variables
        let config = ConfigBuilder::builder()
            // Load from default file
            .add_source(ConfigFile::from(config_path))
            // Load from environment variables with higher priority
            .add_source(
                Environment::with_prefix(ENVIRONMENT_PREFIX)
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?;

        // Deserialize the configuration
        let app_config: AppConfig = config.try_deserialize()?;
        Ok(app_config)
    }
}
