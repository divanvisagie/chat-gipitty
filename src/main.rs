use std::io::{self, Read};
use clap::{Parser, command};

mod chatgpt;

use crate::chatgpt::get_response;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What you want to ask Chat GPT
    #[arg(short, long)]
    query: Option<String>,
}

#[tokio::main]
async fn main() { 
    let args = Args::parse();
    
    if let Some(query) = args.query {
        // Handle the case when the query is not provided
        let response_text = get_response(query).await;
        println!("{}", response_text);
        return
    } else {

        let mut content = String::new();
        io::stdin()
            .read_to_string(&mut content)
            .expect("Failed to read from stdin");

        let response_text = get_response(content).await;

        println!("{}", response_text);
    }
}


