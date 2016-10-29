use protocol;

use std::io;

#[derive(Debug)]
pub enum Error
{
    Protocol(protocol::Error),
}

impl From<protocol::Error> for Error
{
    fn from(e: protocol::Error) -> Self {
        Error::Protocol(e)
    }
}

impl From<io::Error> for Error
{
    fn from(e: io::Error) -> Self {
        Error::Protocol(protocol::Error::Io(e))
    }
}
