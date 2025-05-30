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

    /// Specify a system prompt
    #[arg(short, long)]
    pub system_prompt: Option<String>,

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
    
    /// Show context in human readable table
    #[arg(short, long)]
    pub markdown: bool,

    /// Don't use messages from the session in this request
    #[arg(short, long)]
    pub no_session: bool,

    /// Speak like Jar Jar Binks
    #[arg(short, long)]
    pub jarjar: bool,
    
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
    /// To enable session caching in your terminal, add the following to your .bashrc or .zshrc:
    /// export CGIP_SESSION_NAME=$(uuid) # for each new terminal session to be unique
    /// or
    /// export CGIP_SESSION_NAME=$(date -I) # for a session that will be the same for the entire day
    Session(SessionSubCommand),
    /// Analyze an image using vision models. Use --file to specify the image path.
    Image(ImageSubCommand),
    /// Convert text to speech using OpenAI's TTS models.
    Tts(TtsSubCommand),
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct SessionSubCommand {
    /// Clear the current session context.
    #[arg(short, long)]
    pub clear: bool,

    /// View the current session context
    #[arg(short, long)]
    pub view: bool
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

#[derive(Parser, Debug)]
#[command(author, version, about = "Analyze images using vision models", long_about = None)]
pub struct ImageSubCommand {
    /// Path to the image file to analyze
    #[arg(short, long, required = true)]
    pub file: String,

    /// Additional prompt text to include with the image analysis
    #[arg(index = 1)]
    pub prompt: Option<String>,

    /// Maximum number of tokens in the response
    #[arg(short, long, default_value = "300")]
    pub max_tokens: u32,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Convert text to speech using OpenAI's TTS models", long_about = None)]
pub struct TtsSubCommand {
    /// Text to convert to speech. If not provided, reads from stdin
    #[arg(index = 1)]
    pub text: Option<String>,

    /// TTS model to use
    #[arg(short, long, default_value = "tts-1")]
    pub model: String,

    /// Voice to use for speech synthesis
    #[arg(short, long, default_value = "alloy")]
    pub voice: String,

    /// Output file path for the audio
    #[arg(short, long, default_value = "speech.mp3")]
    pub output: String,

    /// Instructions for the voice (how to speak)
    #[arg(short, long)]
    pub instructions: Option<String>,

    /// Audio format (mp3, opus, aac, flac)
    #[arg(short, long, default_value = "mp3")]
    pub format: String,

    /// Speed of speech (0.25 to 4.0)
    #[arg(short, long, default_value = "1.0")]
    pub speed: f32,
}
