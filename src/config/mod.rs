use serde::{Deserialize, Serialize};
use std::sync::RwLock;
use lazy_static::lazy_static;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Invalid configuration value: {0}")]
    ValidationError(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub api_key: String,
    pub min_value: u32,
    pub detection_threshold: f32,
    pub highlight_enabled: bool,
    pub tooltip_enabled: bool,
    pub highlight_color: [f32; 4],
    pub tooltip_font_size: u32,
    pub tooltip_font_color: [f32; 4],
    pub data_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let mut data_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        data_dir.push("tarkov-price-overlay");
        
        Self {
            api_key: String::new(),
            min_value: 10000,
            detection_threshold: 0.8,
            highlight_enabled: true,
            tooltip_enabled: true,
            highlight_color: [1.0, 0.0, 0.0, 0.5],
            tooltip_font_size: 16,
            tooltip_font_color: [1.0, 1.0, 1.0, 1.0],
            data_dir,
        }
    }
}

impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.api_key.is_empty() {
            return Err(ConfigError::ValidationError("API key cannot be empty".into()));
        }
        if self.min_value < 0 {
            return Err(ConfigError::ValidationError("Minimum value threshold cannot be negative".into()));
        }
        if self.detection_threshold < 0.0 || self.detection_threshold > 1.0 {
            return Err(ConfigError::ValidationError("Detection threshold must be between 0 and 1".into()));
        }
        if self.tooltip_font_size <= 0 {
            return Err(ConfigError::ValidationError("Tooltip font size must be positive".into()));
        }
        Ok(())
    }
}

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
}

impl Config {
    pub fn get() -> Config {
        CONFIG.read().unwrap().clone()
    }

    pub fn set(config: Config) -> Result<(), ConfigError> {
        config.validate()?;
        *CONFIG.write().unwrap() = config;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<(), ConfigError> {
        let path = PathBuf::from(path);
        let config_str = std::fs::read_to_string(&path)?;
        let mut config: Config = serde_json::from_str(&config_str)?;
        config.data_dir = path;
        config.validate()?;
        Self::set(config)
    }

    pub fn save_to_file(&self) -> Result<(), ConfigError> {
        let path = self.data_dir.as_path();
        let config_str = serde_json::to_string_pretty(self)?;
        std::fs::write(path, config_str)?;
        Ok(())
    }

    pub fn get_default_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("tarkov-price-overlay");
        path.push("config.json");
        path
    }
} 