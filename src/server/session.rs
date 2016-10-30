use Credentials;

use std::path::PathBuf;

/// The state of a client.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Session
{
    /// We are waiting for the user to login.
    Pending(Pending),
    /// We are connected and logged in as a user.
    Ready(Ready),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pending
{
    /// We need to send them a welcome message.
    PendingWelcome,
    /// The client needs to initiate login by sending 'USER <name>'.
    WaitingForUsername,
    /// The client has initiated login, and we are waiting for their password.
    WaitingForPassword {
        username: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ready
{
    pub credentials: Credentials,
    pub working_dir: PathBuf,
}

impl Ready
{
    pub fn new(credentials: Credentials) -> Self {
        Ready {
            credentials: credentials,
            working_dir: "/".into(),
        }
    }
}

impl Default for Session
{
    fn default() -> Self {
        Session::Pending(Pending::PendingWelcome)
    }
}
