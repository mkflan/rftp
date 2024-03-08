#![allow(unused)]
#![warn(clippy::pedantic, clippy::nursery, rust_2018_idioms)]

use clap::Parser;
use std::{
    io::{self, Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
};

#[derive(Parser)]
struct Cli {
    /// The IP address to listen on.
    #[arg(short, long)]
    ip: String,

    /// The port to listen on.
    #[arg(short, long)]
    port: u16,
}

fn handle_connection(mut stream: TcpStream) {
    println!("Handling connection.");

    let welcome_msg = "220 rftp server\r\n";
    stream.write_all(welcome_msg.as_bytes()).unwrap();

    let mut buf = [0; 1024];

    loop {
        if let Ok(size) = stream.read(&mut buf) {
            if size == 0 {
                println!("No data read");
                break;
            }

            let cmd = String::from_utf8_lossy(&buf[..size]);
            println!("Recieved command: {cmd}");

            let mut it = cmd.split_whitespace();
            let cmd_name = it.next().unwrap();
            let args = it.next().unwrap_or_default();

            let res = match cmd_name {
                "USER" => "331 Please specify a password.\r\n",
                "PASS" => "230 Login successful.\r\n",
                "QUIT" => "221 Client terminated!\r\n",
                _ => "500 Unknown Command\r\n",
            };

            stream.write_all(res.as_bytes()).unwrap();
        } else {
            println!("Unable to read from TCP stream.");
            break;
        }
    }
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let ip_addr = args
        .ip
        .parse::<IpAddr>()
        .expect("unable to parse IP passed by command line to a valid IP address");
    let sock_addr = SocketAddr::new(ip_addr, args.port);
    let listener = TcpListener::bind(sock_addr).expect("unable to bind to socket");

    println!("Listening on {sock_addr}");

    for conn in listener.incoming() {
        match conn {
            Ok(stream) => handle_connection(stream),
            Err(_) => println!("Unable to handle connection"),
        }
    }

    Ok(())
}
