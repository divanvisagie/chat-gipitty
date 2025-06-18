use reqwest::blocking::Client;
use serde_json::json;
use std::fs::File;
use std::io::Write;

use crate::args::EmbeddingSubCommand;
use crate::utils::get_stdin;

pub fn run(args: &EmbeddingSubCommand) -> Result<(), Box<dyn std::error::Error>> {
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

    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| "Missing OPENAI_API_KEY environment variable")?;

    let payload = json!({
        "input": text,
        "model": args.model,
    });

    let client = Client::new();

    let base_url =
        std::env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com".to_string());
    let url = if base_url.contains("/embeddings") {
        base_url
    } else if base_url.ends_with("/v1") {
        format!("{}/embeddings", base_url)
    } else {
        format!("{}/v1/embeddings", base_url.trim_end_matches('/'))
    };

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

    let resp_json: serde_json::Value = response.json()?;
    let embedding = match resp_json["data"][0]["embedding"].as_array() {
        Some(arr) => arr
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0).to_string())
            .collect::<Vec<_>>()
            .join(","),
        None => {
            eprintln!("Unexpected response format");
            std::process::exit(1);
        }
    };

    if let Some(path) = &args.output {
        let mut file = File::create(path)?;
        file.write_all(embedding.as_bytes())?;
        println!("Embedding saved to: {}", path);
    } else {
        println!("{}", embedding);
    }

    Ok(())
}
