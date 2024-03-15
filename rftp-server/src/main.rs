#![allow(unused)]
#![warn(clippy::pedantic, clippy::nursery, rust_2018_idioms)]

mod state;

use crate::state::ServerState;
use std::{
    io::{self, Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
};

/// Handle a connection from a FTP client.
fn handle_connection(state: &mut ServerState) {
    println!("Handling connection.");

    let welcome_msg = "220 rftp server\r\n";
    state.write_ctrl(welcome_msg.as_bytes()).unwrap();

    loop {
        if let Ok(cmd) = state.read_ctrl() {
            println!("Recieved command: {cmd}");

            let mut it = cmd.split_whitespace();
            let cmd_name = it.next().unwrap();
            let args = it.next().unwrap_or_default();

            let res = match cmd_name {
                "USER" => "331 Please specify a password.\r\n",
                "PASS" => "230 Login successful.\r\n",
                "QUIT" => "221 Client terminated!\r\n",
                "PORT" => todo!(),
                "TYPE" => todo!(),
                "MODE" => todo!(),
                "STRU" => todo!(),
                "RETR" => todo!(),
                "STOR" => todo!(),
                "HELP" => "214 Server help.\r\n",
                "NOOP" => todo!(),
                _ => "500 Unknown Command\r\n",
            };

            state.write_ctrl(res.as_bytes()).unwrap();
        } else {
            println!("Unable to read from control connection.");
            break;
        }
    }
}

fn main() -> io::Result<()> {
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 2121);
    let listener = TcpListener::bind(sock_addr).expect("unable to bind to socket");

    println!("Listening on {sock_addr}");

    for conn in listener.incoming() {
        match conn {
            Ok(stream) => {
                let mut state = ServerState::new(stream);
                handle_connection(&mut state)
            }
            Err(_) => println!("Unable to handle connection"),
        }
    }

    Ok(())
}
