use dirs::config_dir;

use crate::{args::ConfigSubCommand, config_manager::ConfigManager};

pub fn run(config_subcommand: &ConfigSubCommand) {
    let config_directory = config_dir()
        .expect("Failed to find config directory")
        .join("cgip");

    let mut config_manager = ConfigManager::new(config_directory);

    if let Some(ref set) = config_subcommand.set {
        let parts: Vec<&str> = set.split('=').collect();
        if parts.len() == 2 {
            config_manager.set_config_value(parts[0], parts[1]);
            println!(
                "Configuration set successfully for {} to {}",
                parts[0], parts[1]
            )
        } else {
            println!("Invalid format for setting configuration. Use cgip config --set key=value");
        }
    }
    if let Some(ref get) = config_subcommand.get {
        let value = config_manager.get_config_value(get);
        println!("Configuration for {} is {}", get, value);
    }
}

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
            model: "gpt-3.5-turbo".to_string(),
            show_progress: true,
            show_context: false,
            markdown: false,
            stored_context_length: 20,
            openai_api_key: "".to_string(),
            anthropic_api_key: Some("".to_string()),
        };

        let contents = toml::to_string(&custom_config).expect("Failed to serialize config");
        let mut file = File::create(config_dir_path.join("config.toml")).expect("Failed to create config file");
        file.write_all(contents.as_bytes()).expect("Failed to write to config file");

        let config_manager = ConfigManager::new(config_dir_path.clone());
        assert_eq!(config_manager.get_config_value("model"), "gpt-3.5-turbo");
        assert_eq!(config_manager.get_config_value("show_progress"), "true");
        assert_eq!(config_manager.get_config_value("show_context"), "false");
        assert_eq!(config_manager.get_config_value("markdown"), "false");
        assert_eq!(config_manager.get_config_value("stored_context_length"), "20");
    }
}
