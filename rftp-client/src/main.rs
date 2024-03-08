#![allow(unused)]
#![warn(clippy::pedantic, clippy::nursery, rust_2018_idioms)]

mod commands;

use clap::Parser;
use std::{
    io::{self, BufRead, BufReader, Write},
    net::{IpAddr, SocketAddr, TcpStream},
};

#[derive(Parser)]
struct Cli {
    /// The server's IP address.
    #[arg(short, long)]
    ip: String,

    /// The port to connect to.
    #[arg(short, long)]
    port: u16,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let ip_addr = args
        .ip
        .parse::<IpAddr>()
        .expect("unable to parse IP passed by command line to a valid IP address");
    let sock_addr = SocketAddr::new(ip_addr, args.port);
    let mut stream = TcpStream::connect(sock_addr).expect("unable to connect to socket");

    let mut stream_reader = BufReader::new(stream.try_clone()?);

    println!("Connected to {sock_addr}");

    // Output the server's welcome message.
    let mut welcome_msg = String::new();
    stream_reader.read_line(&mut welcome_msg)?;
    println!("{welcome_msg}");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut cmd = String::new();

        if io::stdin().read_line(&mut cmd).is_ok() {
            let mut it = cmd.split_whitespace();
            let cmd_name = it.next().unwrap();
            let args = it.next().unwrap_or_default();
            let cmd = cmd_name.to_uppercase() + " " + args;
            commands::handle_command(&cmd, &mut stream, &mut stream_reader);
        } else {
            println!("Unable to parse input");
            break;
        }
    }

    Ok(())
}
