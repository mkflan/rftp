use crate::{help, ClientState};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    str::FromStr,
};
use Command::*;

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

    #[rustfmt::skip]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USER" => Ok(USER),
            "PASS" => Ok(PASS),
            "ACCT" => Ok(ACCT),
            "CWD"  => Ok(CWD),
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
            "RMD"  => Ok(RMD),
            "MKD"  => Ok(MKD),
            "PWD"  => Ok(PWD),
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
#[rustfmt::skip]
pub fn handle_command(cmd_full: &str, state: &mut ClientState) {
    let mut it = cmd_full.split_whitespace();
    let cmd_name = it.next().unwrap();
    let args = it.next().unwrap_or_default();
    let cmd = Command::from_str(cmd_name).unwrap();

    match cmd {
        USER => state.write_ctrl(cmd_full.as_bytes()).unwrap(),
        PASS => state.write_ctrl(cmd_full.as_bytes()).unwrap(),
        ACCT => todo!(),
        CWD  => todo!(),
        CDUP => todo!(),
        SMNT => todo!(),
        QUIT => std::process::exit(0),
        REIN => todo!(),
        PORT => todo!(),
        PASV => todo!(),
        TYPE => todo!(),
        STRU => todo!(),
        MODE => todo!(),
        RETR => todo!(),
        STOR => todo!(),
        STOU => todo!(),
        APPE => todo!(),
        ALLO => todo!(),
        REST => todo!(),
        RNFR => todo!(),
        RNTO => todo!(),
        ABOR => todo!(),
        DELE => todo!(),
        RMD  => todo!(),
        MKD  => todo!(),
        PWD  => todo!(),
        LIST => todo!(),
        NLST => todo!(),
        SITE => todo!(),
        SYST => todo!(),
        STAT => todo!(),
        HELP => help::display_help((!args.is_empty()).then_some(args)),
        NOOP => todo!(),
        _ => todo!(),
    }
}
