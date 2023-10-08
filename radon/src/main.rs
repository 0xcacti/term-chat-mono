use clap::{crate_version, Parser, Subcommand};
use radon::server::{RunArgs, Server, ServerConfig};
use std::{env, process};

#[derive(Debug, Parser)]
#[command(name="radon", version=crate_version!(), about="terminal chat server", long_about = "Server to let you chat with friends in the terminal", arg_required_else_help(true))]
struct App {
    /// The subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run(RunArgs),
}

#[tokio::main]
async fn main() {
    let mut config = ServerConfig::from(ServerConfig::figment()).unwrap();

    let args = App::parse();

    // handle commands
    match &args.command {
        Some(Commands::Run(arguments)) => {
            config.merge_with_args(arguments);
            let mut server = Server::new(config).unwrap();
            server.run().await.unwrap();
        }
        None => {
            eprintln!("No command provided");
            process::exit(1);
        }
    }
}