use std::io::prelude::*;
use std::{io, fmt};

use itertools::Itertools;
use byteorder::{ByteOrder, NetworkEndian};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandKind
{
    PORT(PORT),
}

impl CommandKind
{
    /// Reads a command from a buffer.
    pub fn read(read: &mut Read) -> Result<Self, io::Error> {
        let line_bytes: Result<Vec<u8>, _> = read.bytes().take_while(|b| b.as_ref().map(|&b| (b as char) != '\n').unwrap_or(true)).collect();
        let line_bytes = line_bytes?;

        let line_string = String::from_utf8(line_bytes).unwrap();

        // Split the line up.
        let (command_name, payload) = line_string.split_at(line_string.chars().position(|c| c == ' ').expect("no space in line") + 1);

        // We don't want to look at the space character.
        let command_name = &command_name[0..command_name.len()-1];

        let mut payload_reader = io::BufReader::new(io::Cursor::new(payload));

        let command = match command_name {
            "PORT" => Ok(CommandKind::PORT(PORT::read_payload(&mut payload_reader)?)),
            _ => panic!("unknown command: {}", command_name),
        };

        command
    }
}

/// An FTP command.
pub trait Command : Clone + fmt::Debug + PartialEq + Eq
{
    /// Writes the command to a buffer.
    fn write(&self, write: &mut Write) -> Result<(), io::Error> {
        write!(write, "{} ", self.command_name())?;

        self.write_payload(write)?;
        Ok(())
    }

    /// Writes the payload data.
    fn write_payload(&self, write: &mut Write) -> Result<(), io::Error>;

    /// Reads payload data.
    fn read_payload(read: &mut BufRead) -> Result<Self, io::Error>;

    /// Gets the name of the command.
    fn command_name(&self) -> &'static str;

    /// Gets the raw bytes for a command.
    fn bytes(&self) -> Vec<u8> {
        let mut buffer = io::Cursor::new(Vec::new());
        self.write(&mut buffer).expect("IO failure while writing to memory buffer");
        buffer.into_inner()
    }
}

/// Sets up an IPv4 port
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PORT
{
    /// The IPv4 address of the host.
    pub host_address: [u8; 4],
    /// The port number.
    pub port: u16,
}

impl Command for PORT
{
    fn write_payload(&self, write: &mut Write) -> Result<(), io::Error> {
        let mut port_buf = [0; 2];
        NetworkEndian::write_u16(&mut port_buf, self.port);

        let address_str = self.host_address.iter().map(|b| b.to_string()).join(",");
        let port_str = port_buf.iter().map(|b| b.to_string()).join(",");

        write!(write, "{},{}", address_str, port_str)
    }

    fn read_payload(read: &mut BufRead) -> Result<Self, io::Error> {
        let mut payload = String::new();
        read.read_to_string(&mut payload)?;

        let textual_bytes: Vec<_> = payload.split(",").collect();
        assert_eq!(textual_bytes.len(), 6, "there should be 6 bytes in a PORT payload");

        let bytes: Vec<u8> = textual_bytes.into_iter().map(|b| b.parse().unwrap()).collect();
        let host = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let port = NetworkEndian::read_u16(&bytes[4..6]);

        Ok(PORT { host_address: host, port: port })
    }

    fn command_name(&self) -> &'static str { "PORT" }
}

#[cfg(test)]
mod test
{
    mod port {
        use raw::*;
        use std::io;

        #[test]
        fn correctly_writes_basic_packets() {
            let packet = PORT { host_address: [127,0,0,1], port: 22 };
            let raw_bytes = packet.bytes();
            let text = String::from_utf8(raw_bytes).unwrap();

            assert_eq!(text, "PORT 127,0,0,1,0,22");
        }

        #[test]
        fn correctly_reads_basic_packets() {
            let raw_bytes = "PORT 192,168,1,1,255,255".as_bytes();
            let command = CommandKind::read(&mut io::Cursor::new(raw_bytes.to_vec())).unwrap();

            assert_eq!(command, CommandKind::PORT(PORT { host_address: [192,168,1,1], port: 65535 }));
        }
    }
}

