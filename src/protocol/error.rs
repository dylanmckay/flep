use reply::{Code, code};
use std::{io, string};

#[derive(Debug)]
pub enum Error
{
    Client(ClientError),
    Io(io::Error),
    FromUtf8(string::FromUtf8Error),
}

#[derive(Debug)]
pub enum ClientError
{
    /// The client sent an invalid command.
    InvalidCommand { name: String },
    /// The client is not logged in.
    NotLoggedIn,
}

impl ClientError
{
    pub fn reply_code(&self) -> Code {
        match *self {
            ClientError::InvalidCommand { .. } => code::INVALID_COMMAND,
            ClientError::NotLoggedIn { .. } => code::USER_NOT_LOGGED_IN,
        }
    }

    pub fn message(&self) -> &'static str {
        match *self {
            ClientError::InvalidCommand { .. } => "invalid command",
            ClientError::NotLoggedIn { .. } => "not logged in",
        }
    }
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
