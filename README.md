# rftp
A simple FTP client and server implemented in Rust. It complies to the FTP specification created through [RFC 959](https://www.rfc-editor.org/rfc/rfc959.html).

## Usage

First, clone the repository onto your system.

To start up the FTP server, run the following command:
```
cargo run --bin rftps
```

To start up the FTP client, run the following command:
```
cargo run --bin rftpc
```

By default, the server will bind to `localhost` port `2121`. Similarly, the client will attempt to connect to the same IP and port.
