# rftp
A simple FTP client and server implemented in Rust. It complies to the FTP specification created through [RFC 959](https://www.rfc-editor.org/rfc/rfc959.html).

## Usage

First, clone the repository onto your system.

To start up the FTP server, run the following command:
```
cargo run start-server
```

To start up the FTP client, run the following command:
```
cargo run start-client
```

If you need more information regarding the flags and arguments either command accepts, pass `--help`.
