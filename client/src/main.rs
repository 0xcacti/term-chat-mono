use clap::{crate_version, Parser, Subcommand};
use client::Config;
use figment::{providers::Env, Figment};
use std::{env, process};

/// term-chat client ui
#[derive(Debug, Parser)]
#[command(name="term-chat-tui-client", version=crate_version!(), about="terminal chat client", long_about = "Chat with friends in the terminal", arg_required_else_help(true))]
struct TermChatParser {
    /// The subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Authenticate user
    Auth {
        /// The username to authenticate with
        #[arg(required = true, short, long)]
        username: String,
        /// The password to authenticate with
        #[arg(required = true, short, long)]
        password: String,
    },
}

#[tokio::main]
async fn main() {
    let mut config: Config = Figment::new()
        .merge(Env::prefixed("BIBLE_RS_"))
        .extract()
        .unwrap();

    let args = TermChatParser::parse();

    // handle commands
    match &args.command {
        Some(Commands::Auth { username, password }) => {
            config.username = username.clone();
            config.password = password.clone();
        }
        None => {
            eprintln!("No command provided");
            process::exit(1);
        }
    }
}
