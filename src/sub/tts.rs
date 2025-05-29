use reqwest::blocking::Client;
use serde_json::json;
use std::fs::File;
use std::io::Write;

use crate::args::TtsSubCommand;
use crate::utils::get_stdin;

pub fn run(args: &TtsSubCommand) -> Result<(), Box<dyn std::error::Error>> {
    // Get text input from args or stdin
    let stdin_text = get_stdin();
    let text = match (stdin_text.is_empty(), &args.text) {
        (true, None) => {
            eprintln!("Error: No text provided. Please provide text as an argument or via stdin.");
            std::process::exit(1);
        }
        (true, Some(arg_text)) => arg_text.clone(),
        (false, None) => stdin_text,
        (false, Some(arg_text)) => format!("{} {}", stdin_text, arg_text),
    };

    // Validate speed parameter
    if args.speed < 0.25 || args.speed > 4.0 {
        eprintln!("Error: Speed must be between 0.25 and 4.0");
        std::process::exit(1);
    }

    // Validate format
    let valid_formats = ["mp3", "opus", "aac", "flac"];
    if !valid_formats.contains(&args.format.as_str()) {
        eprintln!("Error: Format must be one of: mp3, opus, aac, flac");
        std::process::exit(1);
    }

    // Get API key
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| "Missing OPENAI_API_KEY environment variable")?;

    // Build request payload
    let mut payload = json!({
        "model": args.model,
        "input": text,
        "voice": args.voice,
        "response_format": args.format,
        "speed": args.speed
    });

    // Add instructions if provided
    if let Some(ref instructions) = args.instructions {
        payload["instructions"] = json!(instructions);
    }

    // Create HTTP client
    let client = Client::new();

    // Get base URL and construct endpoint
    let base_url =
        std::env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com".to_string());

    let url = if base_url.contains("/audio/speech") {
        base_url
    } else if base_url.ends_with("/v1") {
        format!("{}/audio/speech", base_url)
    } else {
        format!("{}/v1/audio/speech", base_url.trim_end_matches('/'))
    };

    println!("Generating speech...");

    // Make API request
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()?;

    if !response.status().is_success() {
        let error_text = response.text()?;
        eprintln!("API Error: {}", error_text);
        std::process::exit(1);
    }

    // Get audio data
    let audio_data = response.bytes()?;

    // Write to output file
    let mut file = File::create(&args.output)?;
    file.write_all(&audio_data)?;

    println!("Audio saved to: {}", args.output);
    Ok(())
}
