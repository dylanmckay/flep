use {Credentials, Error, FileType};
use io::DataTransferMode;
use {server, protocol};

use std::net::SocketAddr;
use std::path::PathBuf;

/// The state of a client.
#[derive(Clone, Debug)]
pub enum Session
{
    /// We need to send them a welcome message.
    PendingWelcome,
    /// We are waiting for (or in the middle of) the user to login.
    Login(Login),
    /// We are connected and logged in as a user.
    Ready(Ready),
}

/// The state of a client that is in the login process.
#[derive(Clone, Debug)]
pub enum Login
{
    /// The client needs to initiate login by sending 'USER <name>'.
    WaitingForUsername,
    /// The client has initiated login, and we are waiting for their password.
    WaitingForPassword {
        username: String,
    },
}

/// The state of a client that is connected and ready for work.
#[derive(Clone, Debug)]
pub struct Ready
{
    /// The credentials of the current user.
    pub credentials: Credentials,
    /// The current working directory.
    pub working_dir: PathBuf,
    /// The current data transfer file mode.
    pub transfer_type: FileType,
    /// Whether the connection is active or passive.
    pub data_transfer_mode: DataTransferMode,

    /// The port given by the `PORT` command.
    /// Can be empty if passive mode is used.
    pub client_addr: Option<SocketAddr>,
    /// The data transfer operations we have queued.
    pub active_transfer: Option<server::Transfer>,
}

impl Session
{
    pub fn expect_ready(&self) -> Result<&Ready, Error> {
        if let Session::Ready(ref ready) = *self {
            Ok(ready)
        } else {
            Err(protocol::Error::from_kind(protocol::ErrorKind::NotLoggedIn.into()).into())
        }
    }

    pub fn expect_ready_mut(&mut self) -> Result<&mut Ready, Error> {
        if let Session::Ready(ref mut ready) = *self {
            Ok(ready)
        } else {
            Err(protocol::Error::from_kind(protocol::ErrorKind::NotLoggedIn.into()).into())
        }
    }

    pub fn expect_login(&self) -> Result<&Login, Error> {
        if let Session::Login(ref login) = *self {
            Ok(login)
        } else {
            // FIXME: return a more appropriate error.
            Err(protocol::Error::from_kind(protocol::ErrorKind::NotLoggedIn.into()).into())
        }
    }
}

impl Ready
{
    /// Creates a new thing.
    pub fn new(credentials: Credentials) -> Self {
        Ready {
            credentials: credentials,
            working_dir: "/".into(),
            transfer_type: FileType::Binary,
            data_transfer_mode: DataTransferMode::default(),
            client_addr: None,
            active_transfer: None,
        }
    }
}

impl Default for Session
{
    fn default() -> Self { Session::PendingWelcome }
}
