use std::env;
use std::process::Command;

use crate::args::AgentSubCommand;
use crate::chatgpt::{GptClient, Message, MessageContent, Role};
use crate::utils::get_file_contents_from_path;

fn run_shell_command(cmd: &str) -> String {
    match Command::new("sh").arg("-c").arg(cmd).output() {
        Ok(output) => {
            let mut text = String::new();
            text.push_str(&String::from_utf8_lossy(&output.stdout));
            text.push_str(&String::from_utf8_lossy(&output.stderr));
            if text.is_empty() {
                "[no output]\n".to_string()
            } else {
                text
            }
        }
        Err(e) => format!("Command failed: {}", e),
    }
}

pub fn run(args: &AgentSubCommand, client: &mut GptClient) {
    if let Err(e) = env::set_current_dir(&args.directory) {
        eprintln!("Failed to change directory: {}", e);
        std::process::exit(1);
    }

    let system_message = "You can run shell commands using the `execute` tool.\
 Use it whenever running a command will help you complete the user's instruction.\
 Once you have the information you need, return a final answer.";
    client.add_message(Role::System, system_message.to_string());

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

    let mut executed: Vec<(String, String)> = Vec::new();
    let mut actions = 0usize;

    let max_actions = args.max_actions;

    loop {
        let resp = client.complete_with_tools(tools.clone());
        let choice = &resp["choices"][0];
        let finish_reason = choice["finish_reason"].as_str().unwrap_or("");
        if let Some(text) = choice["message"]["content"].as_str() {
            if !text.trim().is_empty() {
                println!("{}", text.trim());
            }
        }

        if finish_reason == "tool_calls" {
            if let Some(calls) = choice["message"]["tool_calls"].as_array() {
                for call in calls {
                    if let Some(args_str) = call["function"]["arguments"].as_str() {
                        let parsed: serde_json::Value =
                            serde_json::from_str(args_str).unwrap_or_default();
                        let command = parsed["command"].as_str().unwrap_or("");
                        let output = run_shell_command(command);
                        executed.push((command.to_string(), output.clone()));
                        if !output.trim().is_empty() {
                            println!("{}", output.trim());
                        }
                        let call_id = call["id"].as_str().unwrap_or("");
                        client.messages.push(Message {
                            role: "tool".to_string(),
                            name: None,
                            tool_call_id: Some(call_id.to_string()),
                            tool_calls: None,
                            content: MessageContent::Text(output),
                        });
                        actions += 1;
                        if actions >= max_actions {
                            println!("Reached maximum actions ({})", max_actions);
                            break;
                        }
                    }
                }
            }
            if actions >= max_actions {
                if !executed.is_empty() {
                    println!("\nCommand summary:");
                    for (cmd, out) in &executed {
                        let first_line = out.lines().next().unwrap_or("");
                        println!("$ {} -> {}", cmd, first_line);
                    }
                }
                break;
            }
        } else {
            if !executed.is_empty() {
                println!("\nCommand summary:");
                for (cmd, out) in &executed {
                    let first_line = out.lines().next().unwrap_or("");
                    println!("$ {} -> {}", cmd, first_line);
                }
            }
            break;
        }
    }
}
