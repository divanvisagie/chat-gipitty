use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(
    author, 
    version, 
    about, 
    long_about = r###"
cgip is a command-line tool that leverages OpenAI's models to address and 
respond to user queries. It compiles context for these queries by prioritizing 
input in a specific order: stdin, command-line arguments, and then files which 
require you using the -f flag. 

The config file will be located in your config directory under cgip/config.toml.
Please edit this file or use `cgip config --set key=value` to set your configuration.

Usage examples:
- To pipe input in from ls, run `ls | cgip "what can you tell me about these files?"`
  This will pipe the output of ls in as the first bit of context and then add the user
  query to the context.

"###
)]
pub struct Args {
    /// Optional. The primary query to sent to the model. 
    /// This is added to the context after stdin and file input.
    #[arg(index=1)]
    pub query: Option<String>,

    /// Read query from a file. If query is present this is added to the prompt after query.
    /// If this is present stdin is ignored.
    #[arg(short, long)]
    pub file: Option<String>,

    /// Specify model to use. Defaults to `gpt-4`.
    #[arg(short = 'M', long)]
    pub model: Option<String>,
    
    /// List all the available models. 
    #[arg(short, long)]
    pub list_models:bool,

    /// Show progress indicator (might mess up stdout).
    #[arg(short = 'p', long)]
    pub show_progress: bool,
    
    /// Output the full context used in the query, including chat context with
    /// all assistant and user messages.
    #[arg(short = 'c',long)]
    pub show_context: bool,
    
    //Show context in human readable table
    #[arg(short, long)]
    pub markdown: bool,
    
    #[command(subcommand)]
    pub subcmd: Option<SubCommands>
}

#[derive(Parser, Debug)]
pub enum SubCommands {
    /// Render the context without running a query against the model.
    View(ViewSubCommand),
    /// Set or get default configuration values with your config.toml.
    Config(ConfigSubCommand),
    /// Used for continuous chat session management and shell integration.
    Session(SessionSubCommand),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct SessionSubCommand {
    /// Used for creating unique session ids for session 
    /// caching, add the following to your .bashrc or .zshrc to enable caching context
    /// within a terminal session.
    /// `export CGIP_SESSION_NAME=$(cgip session -i)`
    #[arg(short, long)]
    pub init: bool,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ViewSubCommand {
    // You can add options and arguments specific to the `view` subcommand here.
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Set or get configuration values", long_about = None)]
pub struct ConfigSubCommand {
    /// Set a configuration value. Use the format key=value.
    /// `cgip config --set model=gpt-4-turbo`
    #[arg(short, long)]
    pub set: Option<String>,

    /// Get your current configuration value. 
    /// `cgip config --get model`
    #[arg(short, long)]
    pub get: Option<String>,
}
