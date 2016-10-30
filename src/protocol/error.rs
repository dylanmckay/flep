use std::{io, string};

#[derive(Debug)]
pub enum Error
{
    /// The client sent an invalid command.
    InvalidCommand { name: String },
    Io(io::Error),
    FromUtf8(string::FromUtf8Error),
}

impl From<io::Error> for Error
{
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<string::FromUtf8Error> for Error
{
    fn from(e: string::FromUtf8Error) -> Self {
        Error::FromUtf8(e)
    }
}
