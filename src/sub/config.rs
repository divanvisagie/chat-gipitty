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

    if config_subcommand.list {
        println!("Current configuration:");
        println!("model: {}", config_manager.get_config_value("model"));
        println!("show_progress: {}", config_manager.get_config_value("show_progress"));
        println!("show_context: {}", config_manager.get_config_value("show_context"));
        println!("markdown: {}", config_manager.get_config_value("markdown"));
        println!(
            "stored_context_length: {}",
            config_manager.get_config_value("stored_context_length")
        );
    }
}

