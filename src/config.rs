use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub window: WindowConfig,
    pub api: ApiConfig,
    pub mcp: McpConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct WindowConfig {
    pub always_on_top: bool,
    pub click_through: bool,
    pub transparent: bool,
    pub default_width: u32,
    pub default_height: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ApiConfig {
    pub enabled: bool,
    pub rate_limit: u32,
    pub auth_required: bool,
    pub api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct McpConfig {
    pub enabled: bool,
    pub log_level: String,
    pub tools_enabled: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                cors_enabled: true,
            },
            window: WindowConfig {
                always_on_top: true,
                click_through: true,
                transparent: true,
                default_width: 800,
                default_height: 600,
            },
            api: ApiConfig {
                enabled: true,
                rate_limit: 100,
                auth_required: false,
                api_key: None,
            },
            mcp: McpConfig {
                enabled: true,
                log_level: "info".to_string(),
                tools_enabled: vec![
                    "add_subtitle".to_string(),
                    "update_subtitle".to_string(),
                    "remove_subtitle".to_string(),
                    "clear_all_subtitles".to_string(),
                    "list_subtitles".to_string(),
                    "toggle_interaction".to_string(),
                    "set_always_on_top".to_string(),
                    "get_status".to_string(),
                ],
            },
        }
    }
}

impl AppConfig {
    #[allow(dead_code)]
    pub fn load() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name("config").required(false))
            .add_source(Environment::with_prefix("SUBTITLE_OVERLAY"))
            .build()?;

        settings.try_deserialize::<AppConfig>()
    }

    #[allow(dead_code)]
    pub fn load_from_file(path: &str) -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name(path).required(true))
            .build()?;

        settings.try_deserialize::<AppConfig>()
    }

    #[allow(dead_code)]
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("subtitle-overlay");
        path.push("config.toml");
        path
    }

    #[allow(dead_code)]
    pub fn ensure_config_dir() -> Result<(), Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("subtitle-overlay");
        
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }
        Ok(())
    }
}

// Add this to Cargo.toml dependencies:
// dirs = "5.0"
// toml = "0.8"
