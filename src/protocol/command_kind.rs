use super::*;

use std::io::prelude::*;
use std::io;

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
    /// Sets the name for the user.
    USER(USER),
}

impl CommandKind
{
    /// Reads a command from a buffer.
    pub fn read(read: &mut Read) -> Result<Self, Error> {
        let line_bytes: Result<Vec<u8>, _> = read.bytes().take_while(|b| b.as_ref().map(|&b| (b as char) != '\n').unwrap_or(true)).collect();
        let mut line_bytes = line_bytes?;

        // Every new line should use '\r\n', and we trimmed the '\n' above.
        assert_eq!(line_bytes.last(), Some(&('\r' as u8)));
        line_bytes.pop();

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
            "USER" => Ok(CommandKind::USER(USER::read_payload(&mut payload_reader)?)),
            _ => panic!("unknown command: {}", command_name),
        };

        command
    }
}
