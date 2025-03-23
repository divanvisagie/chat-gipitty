use crate::client::client::LanguageModelClient;
use args::{Args, SubCommands};
use clap::Parser;

mod args;
mod chat;
mod client;
mod config_manager;
mod printer;
mod sub;
mod utils;

fn main() {
    let args = Args::parse();
    match &args.subcmd {
        Some(SubCommands::Config(config_sc)) => {
            sub::config::run(config_sc);
        }
        Some(SubCommands::Session(subcmd)) => {
            let mut printer = printer::Printer::Console(printer::ConsolePrinter {});
            sub::session::run(subcmd, &mut printer);
        }
        Some(SubCommands::View(_v_sc)) => {
            sub::view::run();
        }
        _ => {
            chat::run(&args);
        }
    }
}

