use std::str::FromStr;

use crate::{
    args::SessionSubCommand,
    client::{Message, Role},
    printer::Printer,
};

use crate::config_manager::ConfigManager;
use anyhow::Result;
use std::{
    env,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

fn get_unique_session_name() -> Result<String> {
    // first check if its in the env
    if let Ok(val) = env::var("CGIP_SESSION_NAME") {
        return Ok(val);
    }
    Err(anyhow::anyhow!("Could not get session name"))
}

pub fn get_tty_file_path() -> Result<PathBuf> {
    let tty = get_unique_session_name()?;

    let tmp_dir = dirs::cache_dir().unwrap();
    let tty_path = tmp_dir.join("cgip");

    if !tty_path.exists() {
        fs::create_dir_all(tty_path.clone())?;
    }

    let tty_path = tty_path.join(tty);
    return Ok(tty_path);
}

pub fn delete_tty_context() {
    let tty_path = match get_tty_file_path() {
        Ok(val) => val,
        Err(_) => {
            return; //just silently exit this function
        }
    };

    if tty_path.exists() {
        fs::remove_file(tty_path).unwrap();
    }
}

pub fn save_to_tty_context(config_manager: &ConfigManager, messages: Vec<Message>) {
    let tty_path = match get_tty_file_path() {
        Ok(val) => val,
        Err(_) => {
            return; //just silently exit this function
        }
    };

    let mut tty_context = if tty_path.exists() {
        let file = File::open(tty_path.clone()).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    } else {
        Vec::new()
    };

    tty_context.extend(messages);

    let max_messages = config_manager.config.stored_context_length;

    // Keep context to a certain length
    if tty_context.len() > max_messages {
        tty_context.remove(0);
    }

    match File::create(tty_path) {
        Ok(file) => {
            serde_json::to_writer(file, &tty_context).unwrap();
        }
        Err(_) => {}
    }
}

pub fn read_from_tty_context() -> Vec<Message> {
    let tty_path = match get_tty_file_path() {
        Ok(val) => val,
        Err(_) => {
            return Vec::new();
        }
    };

    if !tty_path.exists() {
        return Vec::new();
    }

    let file = File::open(tty_path).unwrap();
    let reader = BufReader::new(file);
    let tty_context: Vec<Message> = serde_json::from_reader(reader).unwrap();
    tty_context
}

pub fn run(subcmd: &SessionSubCommand, printer: &mut Printer) {
    if subcmd.view {
        let messages = read_from_tty_context();
        let visible_messages: Vec<Message> = messages
            .iter()
            .cloned()
            .filter(|msg| msg.role != "system")
            .collect();

        for msg in visible_messages {
            let role = Role::from_str(msg.role.as_str()).expect("could not convert role");
            let role_str = role.to_string();
            let content = msg.content;
            printer.print_message(role_str.as_str(), content.as_str());
        }
        return;
    }
    if subcmd.clear {
        delete_tty_context();
        return;
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::client::LanguageModelRequest, printer::MockPrinter, config_manager::ConfigManager};
    use tempfile::TempDir;
    use std::{env, fs};

    use super::*;

    fn setup_test_env() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let session_name = "test_session";
        env::set_var("CGIP_SESSION_NAME", session_name);
        env::set_var("XDG_CACHE_HOME", temp_dir.path());
        temp_dir
    }

    fn cleanup_test_env() {
        env::remove_var("CGIP_SESSION_NAME");
        env::remove_var("XDG_CACHE_HOME");
        delete_tty_context();
    }

    #[test]
    fn test_run_view() {
        let temp_dir = setup_test_env();
        delete_tty_context(); // Ensure clean state

        // Create config directory
        let config_dir = temp_dir.path().join("cgip");
        fs::create_dir_all(&config_dir).unwrap();

        // Create and save messages
        let mut request = LanguageModelRequest::new("test".to_string());
        request.add_message(Role::User, "i have a question".to_string());
        request.add_message(Role::Assistant, "This is a test language model response".to_string());

        let config_manager = ConfigManager::new(config_dir);
        save_to_tty_context(&config_manager, request.messages.clone());

        // View messages
        let subcmd = SessionSubCommand {
            view: true,
            clear: false,
        };
        let mut mp = MockPrinter::new();
        let mut printer = Printer::Mock(&mut mp);

        run(&subcmd, &mut printer);

        // Verify output
        assert_eq!(mp.messages.len(), 2, "Should print exactly 2 messages");
        assert_eq!(mp.messages[0].0, "user", "First message should be from user");
        assert_eq!(mp.messages[0].1, "i have a question", "First message content should match");
        assert_eq!(mp.messages[1].0, "assistant", "Second message should be from assistant");
        assert_eq!(mp.messages[1].1, "This is a test language model response", "Second message content should match");

        cleanup_test_env();
        drop(temp_dir);
    }
}

