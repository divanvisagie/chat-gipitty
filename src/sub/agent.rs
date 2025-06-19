use std::process::Command;
use std::env;

use crate::args::AgentSubCommand;
use crate::chatgpt::{GptClient, Message, MessageContent, Role};
use crate::utils::get_file_contents_from_path;

fn run_shell_command(cmd: &str) -> String {
    match Command::new("sh").arg("-c").arg(cmd).output() {
        Ok(output) => {
            let mut text = String::new();
            text.push_str(&String::from_utf8_lossy(&output.stdout));
            text.push_str(&String::from_utf8_lossy(&output.stderr));
            if text.is_empty() {"[no output]\n".to_string()} else {text}
        }
        Err(e) => format!("Command failed: {}", e),
    }
}

pub fn run(args: &AgentSubCommand, client: &mut GptClient) {
    if let Err(e) = env::set_current_dir(&args.directory) {
        eprintln!("Failed to change directory: {}", e);
        std::process::exit(1);
    }

    if let Some(files) = &args.input {
        for file in files {
            let content = get_file_contents_from_path(file.to_string());
            client.add_message(Role::User, content);
        }
    }

    client.add_message(Role::User, args.instruction.clone());

    let tools = serde_json::json!([
        {
            "type": "function",
            "function": {
                "name": "execute",
                "description": "Run a shell command and return stdout and stderr",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "command": {"type": "string"}
                    },
                    "required": ["command"]
                }
            }
        }
    ]);

    loop {
        let resp = client.complete_with_tools(tools.clone());
        let choice = &resp["choices"][0];
        let finish_reason = choice["finish_reason"].as_str().unwrap_or("");
        if finish_reason == "tool_calls" {
            if let Some(calls) = choice["message"]["tool_calls"].as_array() {
                for call in calls {
                    if let Some(args_str) = call["function"]["arguments"].as_str() {
                        let parsed: serde_json::Value = serde_json::from_str(args_str).unwrap_or_default();
                        let command = parsed["command"].as_str().unwrap_or("");
                        let output = run_shell_command(command);
                        client.messages.push(Message {
                            role: "tool".to_string(),
                            content: MessageContent::Text(output),
                        });
                    }
                }
            }
        } else {
            if let Some(text) = choice["message"]["content"].as_str() {
                println!("{}", text);
            }
            break;
        }
    }
}
