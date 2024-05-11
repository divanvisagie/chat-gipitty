use std::{fs::File, io::Write, path::PathBuf};

use config::{Config, File as ConfigFile, FileFormat};
use serde::{Deserialize, Serialize};

use crate::utils::ensure_config_file;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub model: String,
    pub show_progress: bool,
    pub show_context: bool,
    pub markdown: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            model: "gpt-4".to_string(),
            show_progress: false,
            show_context: false,
            markdown: false,
        }
    }
}

pub struct ConfigManager {
    pub config: AppConfig,
    pub config_directory: PathBuf,
}

impl ConfigManager {
    pub fn new(config_directory: PathBuf) -> Self {
        Self::setup_config(&config_directory);
        let config = Self::load_config(&config_directory);
        ConfigManager {
            config,
            config_directory,
        }
    }

    pub fn setup_config(dir: &PathBuf) {
        if let Err(e) = ensure_config_file(dir) {
            panic!("Failed to ensure config file exists: {}", e);
        }
    }

    pub fn load_config(dir: &PathBuf) -> AppConfig {
        let config_path = dir.join("config.toml");
        let defaults = Config::try_from(&AppConfig::default()).unwrap();
        let config = Config::builder() // sources will be merged by priority
            .add_source(defaults)
            .add_source(ConfigFile::new(
                config_path.to_str().unwrap(),
                FileFormat::Toml,
            ))
            .build()
            .unwrap();
        let loaded_config = config
            .try_deserialize::<AppConfig>()
            .expect("Failed to deserialize config");

        loaded_config
    }

    pub fn set_config_value(&mut self, key: &str, value: &str) {
        let config_path = match ensure_config_file(&self.config_directory) {
            Ok(path) => path,
            Err(e) => panic!("Failed to ensure config file exists: {}", e),
        };

        let mut config = if self.config_directory.exists() {
            Self::load_config(&self.config_directory)
        } else {
            AppConfig::default()
        };

        match key {
            "model" => config.model = value.to_string(),
            "show_progress" => {
                config.show_progress = value.parse().expect("Invalid value for show_progress")
            }
            "show_context" => {
                config.show_context = value.parse().expect("Invalid value for show_context")
            }
            "markdown" => config.markdown = value.parse().expect("Invalid value for markdown"),
            _ => eprintln!("Invalid configuration key"),
        }

        let contents = toml::to_string(&config).expect("Failed to serialize config");
        let mut file = File::create(&config_path).expect("Failed to create config file");
        file.write_all(contents.as_bytes())
            .expect("Failed to write to config file");
    }

    pub fn get_config_value(&self, key: &str) -> String {
        match key {
            "model" => self.config.model.clone(),
            "show_progress" => self.config.show_progress.to_string(),
            "show_context" => self.config.show_context.to_string(),
            "markdown" => self.config.markdown.to_string(),
            _ => "Invalid configuration key".to_string(),
        }
    }
}
