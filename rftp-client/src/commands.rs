use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    process::exit,
    str::FromStr,
};

/// All FTP commands specified by RFC 959.
#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
enum Command {
    USER,
    PASS,
    ACCT,
    CWD,
    CDUP,
    SMNT,
    QUIT,
    REIN,
    PORT,
    PASV,
    TYPE,
    STRU,
    MODE,
    RETR,
    STOR,
    STOU,
    APPE,
    ALLO,
    REST,
    RNFR,
    RNTO,
    ABOR,
    DELE,
    RMD,
    MKD,
    PWD,
    LIST,
    NLST,
    SITE,
    SYST,
    STAT,
    HELP,
    NOOP,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[allow(clippy::enum_glob_use)]
        use Command::*;

        match s {
            "USER" => Ok(USER),
            "PASS" => Ok(PASS),
            "ACCT" => Ok(ACCT),
            "CWD" => Ok(CWD),
            "CDUP" => Ok(CDUP),
            "SMNT" => Ok(SMNT),
            "QUIT" => Ok(QUIT),
            "REIN" => Ok(REIN),
            "PORT" => Ok(PORT),
            "PASV" => Ok(PASV),
            "TYPE" => Ok(TYPE),
            "STRU" => Ok(STRU),
            "MODE" => Ok(MODE),
            "RETR" => Ok(RETR),
            "STOR" => Ok(STOR),
            "STOU" => Ok(STOU),
            "APPE" => Ok(APPE),
            "ALLO" => Ok(ALLO),
            "REST" => Ok(REST),
            "RNFR" => Ok(RNFR),
            "RNTO" => Ok(RNTO),
            "ABOR" => Ok(ABOR),
            "DELE" => Ok(DELE),
            "RMD" => Ok(RMD),
            "MKD" => Ok(MKD),
            "PWD" => Ok(PWD),
            "LIST" => Ok(LIST),
            "NLST" => Ok(NLST),
            "SITE" => Ok(SITE),
            "SYST" => Ok(SYST),
            "STAT" => Ok(STAT),
            "HELP" => Ok(HELP),
            "NOOP" => Ok(NOOP),
            _ => Err(()),
        }
    }
}

/// Handle a command.
pub fn handle_command(cmd: &str, stream: &mut TcpStream, stream_reader: &mut BufReader<TcpStream>) {
    // Send the command to the server.
    stream.write_all(cmd.as_bytes()).unwrap();

    let mut response = String::new();
    stream_reader.read_line(&mut response).unwrap();
    println!("{response}");

    // Check the server's response to see if the command was invalid.
    if response.starts_with("500") {
        return;
    }

    // If we haven't returned, we have a valid command.
    let mut it = cmd.split_whitespace();
    let cmd_name = it.next().unwrap();
    let args = it.next().unwrap_or_default();
    let cmd = Command::from_str(cmd_name).unwrap();

    match cmd {
        Command::QUIT => exit(0),
        _ => todo!(),
    }
}
