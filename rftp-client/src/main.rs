#![allow(unused)]
#![warn(clippy::pedantic, clippy::nursery, rust_2018_idioms)]

mod commands;
mod help;

use std::{
    io::{self, BufRead, BufReader, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
};

struct ClientState {
    /// The control connection stream.
    ctrl_stream: TcpStream,
}

impl ClientState {
    fn new(ctrl_stream: TcpStream) -> Self {
        Self { ctrl_stream }
    }

    /// Read from the control stream.
    fn read_ctrl(&self) -> io::Result<String> {
        let mut ctrl_read = BufReader::new(&self.ctrl_stream);
        let mut out = String::new();
        ctrl_read.read_line(&mut out)?;
        Ok(out)
    }

    /// Write to the control stream.
    fn write_ctrl(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.ctrl_stream.write_all(bytes)
    }
}

fn main() -> io::Result<()> {
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 2121);
    let stream = TcpStream::connect(sock_addr).expect("unable to connect to socket");
    let mut state = ClientState::new(stream);

    println!("Connected to {sock_addr}");

    // Output the server's welcome message.
    println!("{}", state.read_ctrl()?);

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut cmd = String::new();

        if io::stdin().read_line(&mut cmd).is_ok() {
            /// Send the command to the server via the control connection.
            state.write_ctrl(cmd.as_bytes())?;

            /// Process the server's response, and determine whether to process a command.
            let response = state.read_ctrl()?;
            println!("{response}");
            let status_code = response[..3].parse::<u32>().unwrap();

            // Skip command handling if command was invalid.
            if status_code == 500 {
                continue;
            }

            commands::handle_command(&cmd, &mut state);
        } else {
            println!("Unable to read input");
            break;
        }
    }

    Ok(())
}
