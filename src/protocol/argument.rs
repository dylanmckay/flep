use std::io::prelude::*;
use std::io;

/// An argument to a command.
pub trait Argument : Sized
{
    fn read(read: &mut BufRead) -> Result<Self, io::Error>;
    fn write(&self, write: &mut Write) -> Result<(), io::Error>;
}

impl Argument for String
{
    fn read(_read: &mut BufRead) -> Result<Self, io::Error> {
        unimplemented!();
    }

    fn write(&self, _write: &mut Write) -> Result<(), io::Error> {
        unimplemented!();
    }
}

