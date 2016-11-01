use reply::{Code, code};
use std::io;

#[derive(Debug)]
pub enum Error
{
    Client(ClientError),
    Server(ServerError),
    Io(io::Error),
}

#[derive(Debug)]
pub enum ClientError
{
    /// The client sent an invalid command.
    InvalidCommand { name: String },
    /// The client is not logged in.
    NotLoggedIn,
    /// A given argument was invalid.
    InvalidArgument { message: String },
}

#[derive(Debug)]
pub enum ServerError
{
    UnimplementedCommand,
}

impl ClientError
{
    pub fn reply_code(&self) -> Code {
        match *self {
            ClientError::InvalidCommand { .. } => code::INVALID_COMMAND,
            ClientError::NotLoggedIn { .. } => code::USER_NOT_LOGGED_IN,
            ClientError::InvalidArgument { .. } => code::SYNTAX_ERROR,
        }
    }

    pub fn message(&self) -> String {
        match *self {
            ClientError::InvalidCommand { ref name } => format!("invalid command: {}", name),
            ClientError::NotLoggedIn { .. } => "not logged in".to_owned(),
            ClientError::InvalidArgument { ref message } => format!("invalid argument: {}", message),
        }
    }
}

impl From<io::Error> for Error
{
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
