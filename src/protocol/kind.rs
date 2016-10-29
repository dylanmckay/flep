use super::*;

use std::io::prelude::*;
use std::{io, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandKind
{
    PORT(PORT),
    /// Abort the current file transfer.
    ABOR(ABOR),
    /// Change directory up one level.
    CDUP(CDUP),
    /// Set the transfer mode.
    MODE(MODE),
    /// A no-operation.
    NOOP(NOOP),
    /// Enable passive mode.
    PASV(PASV),
    /// Gets the name of the current working directory on the remote host.
    PWD(PWD),
    /// Terminates the command connection.
    QUIT(QUIT),
    /// Reinitializes the command connectio.
    REIN(REIN),
    /// Begins transmission of a file to the remote site.
    STOU(STOU),
    /// Returns a word identifying the system.
    SYST(SYST),
}

impl CommandKind
{
    /// Reads a command from a buffer.
    pub fn read(read: &mut Read) -> Result<Self, io::Error> {
        let line_bytes: Result<Vec<u8>, _> = read.bytes().take_while(|b| b.as_ref().map(|&b| (b as char) != '\n').unwrap_or(true)).collect();
        let line_bytes = line_bytes?;

        let line_string = String::from_utf8(line_bytes).unwrap();

        // Split the line up.
        let (command_name, payload) = if line_string.contains(' ') {
            let (command_name, payload) = line_string.split_at(line_string.chars().position(|c| c == ' ').expect("no space in line") + 1);

            // We don't want to look at the space character.
            (&command_name[0..command_name.len()-1], payload)
        } else {
            // If the line has no space, it has no payload.
            (line_string.as_str(), "")
        };

        let mut payload_reader = io::BufReader::new(io::Cursor::new(payload));

        let command = match command_name {
            "PORT" => Ok(CommandKind::PORT(PORT::read_payload(&mut payload_reader)?)),
            "ABOR" => Ok(CommandKind::ABOR(ABOR::read_payload(&mut payload_reader)?)),
            "CDUP" => Ok(CommandKind::CDUP(CDUP::read_payload(&mut payload_reader)?)),
            "MODE" => Ok(CommandKind::MODE(MODE::read_payload(&mut payload_reader)?)),
            "NOOP" => Ok(CommandKind::NOOP(NOOP::read_payload(&mut payload_reader)?)),
            "PASV" => Ok(CommandKind::PASV(PASV::read_payload(&mut payload_reader)?)),
            "PWD" => Ok(CommandKind::PWD(PWD::read_payload(&mut payload_reader)?)),
            "QUIT" => Ok(CommandKind::QUIT(QUIT::read_payload(&mut payload_reader)?)),
            "REIN" => Ok(CommandKind::REIN(REIN::read_payload(&mut payload_reader)?)),
            "STOU" => Ok(CommandKind::STOU(STOU::read_payload(&mut payload_reader)?)),
            "SYST" => Ok(CommandKind::SYST(SYST::read_payload(&mut payload_reader)?)),
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
        // Write the payload to a temporary space
        let mut payload_buffer = io::Cursor::new(Vec::new());
        self.write_payload(&mut payload_buffer)?;
        let payload = payload_buffer.into_inner();

        // Don't write a redundant space unless there actually is a payload.
        if payload.is_empty() {
            write!(write, "{}", self.command_name())?;
        } else {
            write!(write, "{} ", self.command_name())?;
            write.write(&payload)?;
        }

        Ok(())
    }

    /// Writes the payload data.
    fn write_payload(&self, write: &mut Write) -> Result<(), io::Error>;

    /// Reads payload data.
    fn read_payload(read: &mut BufRead) -> Result<Self, io::Error>;

    /// Gets the name of the command.
    fn command_name(&self) -> &'static str;

    fn bytes(&self) -> Vec<u8> {
        let mut buffer = io::Cursor::new(Vec::new());
        self.write(&mut buffer).expect("IO failure while writing to memory buffer");
        buffer.into_inner()
    }

    /// Generates the text string for this packet.
    fn to_string(&self) -> String {
        String::from_utf8(self.bytes()).unwrap()
    }
}
