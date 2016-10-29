pub use self::port::PORT;

pub mod port;

use std::io::prelude::*;
use std::{io, fmt};

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
