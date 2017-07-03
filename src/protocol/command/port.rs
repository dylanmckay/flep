use {Command, Error, ErrorKind};
use std::io::prelude::*;

use itertools::Itertools;
use byteorder::{ByteOrder, NetworkEndian};

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
    fn write_payload(&self, write: &mut Write) -> Result<(), Error> {
        let mut port_buf = [0; 2];
        NetworkEndian::write_u16(&mut port_buf, self.port);

        let address_str = self.host_address.iter().map(|b| b.to_string()).join(",");
        let port_str = port_buf.iter().map(|b| b.to_string()).join(",");

        write!(write, "{},{}", address_str, port_str)?;
        Ok(())
    }

    fn read_payload(read: &mut BufRead) -> Result<Self, Error> {
        let mut payload = String::new();
        read.read_to_string(&mut payload)?;

        assert_eq!(payload.chars().next(), Some(' '), "missing space after command");
        let payload: String = payload.chars().skip(1).collect();

        let textual_bytes: Vec<&str> = payload.split(",").collect();
        assert_eq!(textual_bytes.len(), 6, "there should be 6 bytes in a PORT payload");

        let bytes: Result<Vec<u8>, _> = textual_bytes.into_iter().map(|tb| match tb.parse() {
            Ok(b) => Ok(b),
            Err(..) => Err(Error::from_kind(ErrorKind::InvalidArgument(
                format!("PORT addresses should be comma-separated integers")))),
        }).collect();
        let bytes = bytes?;

        let host = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let port = NetworkEndian::read_u16(&bytes[4..6]);

        Ok(PORT { host_address: host, port: port })
    }

    fn command_name(&self) -> &'static str { "PORT" }
}

#[cfg(test)]
mod test
{
    use {CommandKind, Command};
    use super::*;
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
        let raw_bytes = "PORT 192,168,1,1,255,255\r\n".as_bytes();
        let command = CommandKind::read(&mut io::Cursor::new(raw_bytes.to_vec())).unwrap();

        assert_eq!(command, CommandKind::PORT(PORT { host_address: [192,168,1,1], port: 65535 }));
    }
}

