use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(
    author, 
    version, 
    about, 
    long_about = r###"
Queries can be piped in through stdin and will be added to the context
first. 

The query passed in as the first argument will then be added to the context
second.

Files read using -f will be added last to the context.

Usage examples:
- To pipe input in from ls, run `ls | cgip "what can you tell me about these files?"`
  This will pipe the output of ls in as the first bit of context and then add the user
  query to the context.

More detailed information can go here.
"###
)]
pub struct Args {
    /// What you want to ask Chat GPT. Query is optional but is added to the prompt
    /// after the contents of the file or stdin if present.
    #[arg(index=1)]
    pub query: Option<String>,

    /// Read query from a file. If query is present this is added to the prompt after query.
    /// If this is present stdin is ignored.
    #[arg(short, long)]
    pub file: Option<String>,

    /// The model to use. Defaults to `gpt-4`
    #[arg(short = 'o', long)]
    pub model: Option<String>,
    
    /// List the available models
    #[arg(short = 'l', long)]
    pub list_models:bool,

    /// Show progress indicator (might mess up stdout)
    #[arg(short = 'p', long)]
    pub show_progress: bool,
    
    /// Prints not only the output from OpenAI but the chat context with all
    /// assistant and user messages.
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
    View(ViewSubCommand)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ViewSubCommand {
    // You can add options and arguments specific to the `view` subcommand here.
}
