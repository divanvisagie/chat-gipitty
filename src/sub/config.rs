use crate::{args::ConfigSubCommand};
use crate::config_manager::ConfigManager;

pub fn run(
    config_manager: &mut ConfigManager,
    config_subcommand: &ConfigSubCommand,
) {
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
