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
    pub stored_context_length: usize
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            model: "gpt-4o".to_string(),
            show_progress: false,
            show_context: false,
            markdown: false,
            stored_context_length: 20
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
            "stored_context_length" => self.config.stored_context_length.to_string(),
            _ => "Invalid configuration key".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    use crate::config_manager::AppConfig;

    #[test]
    fn test_custom_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir_path = temp_dir.path().join("cgip");
        let mut config_manager = ConfigManager::new(config_dir_path.clone());

        // Manually create a custom config
        let custom_config = AppConfig {
            model: "gpt-4o".to_string(),
            show_progress: true,
            show_context: false,
            markdown: false,
            stored_context_length: 20
        };

        // Serialize and save this custom config
        let config_path = config_dir_path.join("config.toml");
        let contents = toml::to_string(&custom_config).expect("Failed to serialize custom config");
        let mut file = File::create(&config_path).expect("Failed to open config file");
        file.write_all(contents.as_bytes())
            .expect("Failed to write custom config to file");

        // Reload config from file
        config_manager.config = ConfigManager::load_config(&config_dir_path);

        assert_eq!(
            config_manager.config.model, "gpt-3.5-turbo",
            "Model should be 'gpt-3.5-turbo'"
        );
        assert_eq!(
            config_manager.config.show_progress, true,
            "show_progress should be true"
        );
    }

    #[test]
    fn test_custom_config_with_missing() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir_path = temp_dir.path().join("cgip");
        let mut config_manager = ConfigManager::new(config_dir_path.clone());

        // Create a partial config file manually
        let config_path = config_dir_path.join("config.toml");
        let contents = "show_progress = true";
        let mut file = File::create(&config_path).expect("Failed to open config file");
        file.write_all(contents.as_bytes())
            .expect("Failed to write partial config to file");

        // Reload config from file
        config_manager.config = ConfigManager::load_config(&config_dir_path);

        assert_eq!(
            config_manager.config.model, "gpt-4",
            "Model should default to 'gpt-4'"
        );
        assert_eq!(
            config_manager.config.show_progress, true,
            "show_progress should be true"
        );
    }

    #[test]
    fn test_default_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir_path = temp_dir.path().join("cgip");
        let config_manager = ConfigManager::new(config_dir_path);

        assert_eq!(
            config_manager.config.model, "gpt-4",
            "Model should default to 'gpt-4'"
        );
        assert_eq!(
            config_manager.config.show_progress, false,
            "show_progress should default to false"
        );
    }
}
