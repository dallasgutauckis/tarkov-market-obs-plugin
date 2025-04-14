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
    pub min_value_threshold: i32,
    pub highlight_enabled: bool,
    pub tooltip_enabled: bool,
    pub detection_threshold: f64,
    pub highlight_color: [f32; 4],
    pub tooltip_font_size: f32,
    pub tooltip_font_color: [f32; 4],
    pub config_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: "mZbkuqAnVhn3YDV6".to_string(),
            min_value_threshold: 100_000,
            highlight_enabled: true,
            tooltip_enabled: true,
            detection_threshold: 0.8,
            highlight_color: [0.0, 1.0, 0.0, 0.5], // Green with 50% opacity
            tooltip_font_size: 0.5,
            tooltip_font_color: [1.0, 1.0, 1.0, 1.0], // White
            config_path: None,
        }
    }
}

impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.api_key.is_empty() {
            return Err(ConfigError::ValidationError("API key cannot be empty".into()));
        }
        if self.min_value_threshold < 0 {
            return Err(ConfigError::ValidationError("Minimum value threshold cannot be negative".into()));
        }
        if self.detection_threshold < 0.0 || self.detection_threshold > 1.0 {
            return Err(ConfigError::ValidationError("Detection threshold must be between 0 and 1".into()));
        }
        if self.tooltip_font_size <= 0.0 {
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
        config.config_path = Some(path);
        config.validate()?;
        Self::set(config)
    }

    pub fn save_to_file(&self) -> Result<(), ConfigError> {
        let path = self.config_path.as_ref()
            .ok_or_else(|| ConfigError::ValidationError("No config file path set".into()))?;
        let config_str = serde_json::to_string_pretty(self)?;
        std::fs::write(path, config_str)?;
        Ok(())
    }

    pub fn get_default_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("tarkuck");
        path.push("config.json");
        path
    }
} 