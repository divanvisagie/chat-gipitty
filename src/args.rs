use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// What you want to ask Chat GPT. Query is optional but is added to the prompt
    /// before file or stdin if present.
    #[arg(index=1)]
    pub query: Option<String>,

    /// Read query from a file. If query is present this is added to the prompt after query.
    /// If this is present stdin is ignored.
    #[arg(short, long)]
    pub file: Option<String>,

    /// Show progress indicator (might mess up stdout)
    #[arg(short, long)]
    pub progress: bool,

    #[arg(short,long)]
    pub show_context: bool
}

