use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// What you want to ask Chat GPT. Query is optional but is added to the prompt
    /// before file if file is present. If this is present stdin is ignored.
    #[arg(short, long)]
    pub query: Option<String>,

    /// Read query from a file. If query is present this is added to the prompt after query.
    /// If this is present stdin is ignored.
    #[arg(short, long)]
    pub file: Option<String>,

    /// Enter interactive chat mode after asking the initial question.
    /// This will be ignored if reading from stdin.
    #[arg(short, long)]
    pub interactive: bool,

    /// Show progress indicator (might mess up stdout)
    #[arg(short, long)]
    pub progress: bool,
}

