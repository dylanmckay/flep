use raw::Command;
use std::io::prelude::*;
use std::io;

/// Sets up an IPv4 port
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PASV;

impl Command for PASV
{
    fn write_payload(&self, _: &mut Write) -> Result<(), io::Error> { Ok(()) }
    fn read_payload(_: &mut BufRead) -> Result<Self, io::Error> { Ok(PASV) }

    fn command_name(&self) -> &'static str { "PASV" }
}

#[cfg(test)]
mod test
{
    mod port {
        use raw::*;
        use std::io;

        #[test]
        fn correctly_writes_basic_packets() {
            let packet = PASV;
            let raw_bytes = packet.bytes();
            let text = String::from_utf8(raw_bytes).unwrap();

            assert_eq!(text, "PASV");
        }

        #[test]
        fn correctly_reads_basic_packets() {
            let raw_bytes = "PASV".as_bytes();
            let command = CommandKind::read(&mut io::Cursor::new(raw_bytes.to_vec())).unwrap();

            assert_eq!(command, CommandKind::PASV(PASV));
        }
    }
}

