pub use self::port::PORT;
pub use self::mode::{MODE, Mode};
pub use self::basic::{ABOR, CDUP, NOOP, PASV, PWD, QUIT, REIN, STOU, SYST};
pub use self::misc::USER;
pub use self::unimplemented::*;

#[macro_use]
pub mod macros;
pub mod port;
pub mod mode;
/// Commands which take no arguments.
pub mod basic;
pub mod misc;
pub mod unimplemented;

use Error;
use std::io::prelude::*;
use std::{io, fmt};

/// An FTP command.
pub trait Command : Clone + fmt::Debug + PartialEq + Eq
{
    /// Writes the command to a buffer.
    fn write(&self, write: &mut Write) -> Result<(), Error> {
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
    fn write_payload(&self, write: &mut Write) -> Result<(), Error>;

    /// Reads payload data.
    fn read_payload(read: &mut BufRead) -> Result<Self, Error>;

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
