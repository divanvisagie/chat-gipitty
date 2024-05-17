use std::{env, fs::{self, File}, io::BufReader, path::PathBuf, process::Command};
use crate::chatgpt::Message;

fn get_unique_session_name() -> Result<String, ()> {
    // first check if its in the env
    if let Ok(val) = env::var("CGIP_SESSION_NAME") {
        return Ok(val);
    }
    Err(())
}

pub fn get_tty_file_path() -> Result<PathBuf,()> {
    let tty = match get_unique_session_name() {
        Ok(val) => val,
        Err(_) => {
            // fail silently and just dont use the feature
            return Err(());
        }
    };

    let tmp_dir = dirs::cache_dir().unwrap();
    let tty_path = tmp_dir.join("cgip");
    
    if !tty_path.exists() {
        match fs::create_dir_all(tty_path.clone()) {
            Ok(_) => {}
            Err(_) => {
                return Err(());
            }
        }
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

pub fn save_to_tty_context(messages: Vec<Message>) {
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

    // if the cache is more than 20 items long, remove the first item
    if tty_context.len() > 20 {
        tty_context.remove(0);
    }

    match File::create(tty_path) {
        Ok(file) => {
            serde_json::to_writer(file, &tty_context).unwrap();
        }
        Err(_) => {
        }
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
