use protocol;
use std::io;

#[derive(Debug)]
pub enum Error
{
    InvalidCommand { message: String },
    Protocol(protocol::Error),
    Server(::server::Error),
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

impl From<::server::Error> for Error
{
    fn from(e: ::server::Error) -> Self {
        Error::Server(e)
    }
}
