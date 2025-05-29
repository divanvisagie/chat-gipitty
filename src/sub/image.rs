use base64::{engine::general_purpose, Engine as _};
use std::fs;
use std::path::Path;

use crate::args::ImageSubCommand;
use crate::chatgpt::{GptClient, Role};

pub fn run(args: &ImageSubCommand, client: &mut GptClient) {
    // Check if file exists
    if !Path::new(&args.file).exists() {
        eprintln!("Error: Image file '{}' not found", args.file);
        std::process::exit(1);
    }

    // Ensure we're using a vision-capable model
    if !is_vision_model(&client.config_manager.config.model) {
        eprintln!("Warning: Current model '{}' may not support vision. Consider using 'gpt-4o' or 'gpt-4-vision-preview'", client.config_manager.config.model);
        // Override model for this request if it's not vision-capable
        client.config_manager.config.model = "gpt-4o".to_string();
    }

    // Read and encode the image file
    let image_url = match encode_image(&args.file) {
        Ok(encoded) => {
            // Detect image format from file extension
            let extension = Path::new(&args.file)
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("jpeg")
                .to_lowercase();

            let mime_type = match extension.as_str() {
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                "gif" => "image/gif",
                "webp" => "image/webp",
                _ => "image/jpeg", // default fallback
            };

            format!("data:{};base64,{}", mime_type, encoded)
        }
        Err(e) => {
            eprintln!("Error reading image file: {}", e);
            std::process::exit(1);
        }
    };

    // Default prompt if none provided
    let prompt = args
        .prompt
        .clone()
        .unwrap_or_else(|| "What is in this image?".to_string());

    // Add the image message to the client
    client.add_image_message(Role::User, Some(prompt), image_url);

    // Complete the request and print the response
    let response = client.complete_with_max_tokens(Some(args.max_tokens));
    println!("{}", response);
}

fn is_vision_model(model: &str) -> bool {
    model.contains("gpt-4o")
        || model.contains("gpt-4.1")
        || model.contains("gpt-4-vision")
        || model.contains("gpt-4-turbo")
        || model == "gpt-4"
}

fn encode_image(image_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let image_data = fs::read(image_path)?;
    Ok(general_purpose::STANDARD.encode(&image_data))
}
