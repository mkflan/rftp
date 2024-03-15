use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::TcpStream,
};

#[derive(Debug, Clone, Copy, Default)]
pub enum RepresentationType {
    /// ASCII non-print
    #[default]
    AsciiNP,

    /// ASCII Telnet
    AsciiT,

    /// ASCII carriage control
    AsciiCC,

    /// EBCDIC non-print
    ENP,

    /// EBCDIC Telnet
    ET,

    /// EBCDIC carriage control
    ECC,

    Image,

    /// Local byte size
    Bytes(usize),
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TransferMode {
    #[default]
    Stream,

    Block,
    Compressed,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum FileStructure {
    /// File (no record)
    #[default]
    File,

    Record,
    Page,
}

#[derive(Debug)]
pub struct ServerState {
    ctrl_stream: TcpStream,
    pub rep_type: RepresentationType,
    pub transfer_mode: TransferMode,
    pub file_structure: FileStructure,
}

impl ServerState {
    pub fn new(ctrl_stream: TcpStream) -> Self {
        Self {
            ctrl_stream,
            rep_type: RepresentationType::default(),
            transfer_mode: TransferMode::default(),
            file_structure: FileStructure::default(),
        }
    }

    /// Read from the control stream.
    pub fn read_ctrl(&self) -> io::Result<String> {
        let mut ctrl_read = BufReader::new(&self.ctrl_stream);
        let mut out = String::new();
        ctrl_read.read_line(&mut out)?;
        Ok(out)
    }

    /// Write to the control stream.
    pub fn write_ctrl(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.ctrl_stream.write_all(bytes)
    }
}
