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
}

impl Default for Session
{
    fn default() -> Self {
        Session::Pending(Pending::PendingWelcome)
    }
}
