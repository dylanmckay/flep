use {Credentials, FileType, DataTransferMode};
use server;

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

#[derive(Clone, Debug)]
pub struct Ready
{
    /// The credentials of the current user.
    pub credentials: Credentials,
    /// The current working directory.
    pub working_dir: PathBuf,
    /// The current data transfer file mode.
    pub transfer_type: FileType,
    /// The current state of the data connection.
    pub data_connection: DataConnection,
    /// Whether the connection is active or passive.
    pub data_transfer_mode: DataTransferMode,
    /// The port given by the 'PORT' command.
    pub port: Option<u16>,
    /// The data transfer operations we have queued.
    pub active_transfer: Option<server::Transfer>,
}

/// The current state of the data connection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataConnection
{
    /// Not sure if we want a data connection yet.
    None,
    /// We need to send an outbound connection to the client.
    PendingOutbound{ port: u16 },
    /// We need to receive an inbound connection from the client.
    PendingInbound { port: u16},
    /// We have successfully made a data protocol connection.
    Connected,
}

impl Ready
{
    pub fn new(credentials: Credentials) -> Self {
        Ready {
            credentials: credentials,
            working_dir: "/".into(),
            transfer_type: FileType::Binary,
            data_connection: DataConnection::None,
            data_transfer_mode: DataTransferMode::default(),
            port: None,
            active_transfer: None,
        }
    }
}

impl Default for Session
{
    fn default() -> Self { Session::PendingWelcome }
}
