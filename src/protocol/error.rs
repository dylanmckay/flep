use reply::{Code, code};
use std::io;

#[derive(Debug)]
pub enum Error
{
    Client(ClientError),
    Io(io::Error),
}

/// An error that can be told to the client.
#[derive(Debug)]
pub enum ClientError
{
    /// The client sent an invalid command.
    InvalidCommand { name: String },
    /// The client is not logged in.
    NotLoggedIn,
    /// A given argument was invalid.
    InvalidArgument { message: String },
    /// The sequence of commands was invalid.
    InvalidCommandSequence { message: String },
    /// The server doesn't implement a command.
    UnimplementedCommand { name: String, },
}

impl ClientError
{
    pub fn reply_code(&self) -> Code {
        use ClientError::*;

        match *self {
            InvalidCommand { .. } => code::INVALID_COMMAND,
            NotLoggedIn => code::USER_NOT_LOGGED_IN,
            InvalidArgument { .. } => code::SYNTAX_ERROR,
            InvalidCommandSequence { .. } => code::BAD_COMMAND_SEQUENCE,
            UnimplementedCommand { .. } => code::COMMAND_NOT_IMPLEMENTED,
        }
    }

    pub fn message(&self) -> String {
        use ClientError::*;

        match *self {
            InvalidCommand { ref name } => format!("invalid command: {}", name),
            NotLoggedIn { .. } => "not logged in".to_owned(),
            InvalidArgument { ref message } => format!("invalid argument: {}", message),
            InvalidCommandSequence { ref message } => format!("invalid command sequence: {}", message),
            UnimplementedCommand { ref name } => format!("command unimplemented: {}", name),
        }
    }
}

impl Into<Error> for ClientError
{
    fn into(self) -> Error { Error::Client(self) }
}

impl From<io::Error> for Error
{
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
