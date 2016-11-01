use protocol;
use std::io;

/// A flep error.
#[derive(Debug)]
pub enum Error
{
    Protocol(protocol::Error),
    Io(io::Error),
}

impl From<io::Error> for Error
{
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<protocol::Error> for Error
{
    fn from(e: protocol::Error) -> Self {
        Error::Protocol(e)
    }
}
