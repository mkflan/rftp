#[rustfmt::skip]
fn display_cmd_help(cmd_name: &str) {
    let (desc, usage) = match cmd_name {
        "USER" => ("", ""),
        "PASS" => ("", ""),
        "ACCT" => ("", ""),
        "CWD"  => ("", ""),
        "CDUP" => ("", ""),
        "SMNT" => ("", ""),
        "QUIT" => ("", ""),
        "REIN" => ("", ""),
        "PORT" => ("", ""),
        "PASV" => ("", ""),
        "TYPE" => ("", ""),
        "STRU" => ("", ""),
        "MODE" => ("", ""),
        "RETR" => ("", ""),
        "STOR" => ("", ""),
        "STOU" => ("", ""),
        "APPE" => ("", ""),
        "ALLO" => ("", ""),
        "REST" => ("", ""),
        "RNFR" => ("", ""),
        "RNTO" => ("", ""),
        "ABOR" => ("", ""),
        "DELE" => ("", ""),
        "RMD"  => ("", ""),
        "MKD"  => ("", ""),
        "PWD"  => ("", ""),
        "LIST" => ("", ""),
        "NLST" => ("", ""),
        "SITE" => ("", ""),
        "SYST" => ("", ""),
        "STAT" => ("", ""),
        "HELP" => ("Display information about supported commands.", "HELP [command_name]"),
        "NOOP" => ("", ""),
        _ => todo!(),
    };

    println!(
        "
Command {cmd_name}
    Description: {desc}
    Usage: {usage}            
        "
    );
}

pub fn display_help(cmd_name: Option<&str>) {
    if let Some(cmd_name) = cmd_name {
        display_cmd_help(cmd_name);
    } else {
        println!(
            "
Command   Description
USER
PASS
ACCT
CWD
CDUP
SMNT
QUIT
REIN
PORT
PASV
TYPE
STRU
MODE
RETR
STOR
STOU
APPE
ALLO
REST
RNFR
RNTO
ABOR
DELE
RMD
MKD
PWD
LIST
NLST
SITE
SYST
STAT
HELP      Display information about supported commands.
NOOP      Do nothing.
            "
        );
    }
}
