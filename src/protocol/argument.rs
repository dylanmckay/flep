use Error;

use std::io::prelude::*;
use std::ascii::AsciiExt;

/// An argument to a command.
pub trait Argument : Sized
{
    fn read(read: &mut BufRead) -> Result<Self, Error>;
    fn write(&self, write: &mut Write) -> Result<(), Error>;
}

impl Argument for String
{
    fn read(read: &mut BufRead) -> Result<Self, Error> {
        let bytes: Result<Vec<u8>, _> = read.bytes().collect();
        let bytes = bytes?;

        Ok(String::from_utf8(bytes)?)
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        for c in self.chars() { assert!(c.is_ascii(), "only ASCII is supported in FTP"); }
        write!(write, "{}", self)?;
        Ok(())
    }
}

