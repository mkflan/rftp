use std::io;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
    StartClient {
        /// The port to connect to.
        #[arg(short, long, default_value = "2121")]
        port: u16,
    },
    StartServer {
        /// The port to listen on.
        #[arg(short, long, default_value = "2121")]
        port: u16,
    },
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    subcmd: Command,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    match args.subcmd {
        Command::StartClient { port } => rftp_client::start(port),
        Command::StartServer { port } => rftp_server::start(port),
    }
}
