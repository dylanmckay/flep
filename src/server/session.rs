/// The state of a client.
pub enum Session
{
    /// We are waiting for the user to login.
    Pending(Pending),
    /// We are connected and logged in as a user.
    Ready(Ready),
}

pub enum Pending
{
    /// We need to send them a welcome message.
    PendingWelcome,
    /// The client needs to login.
    WaitingForLogin,
    /// The user has logged in.
    LoggedIn,
}

pub struct Ready
{
}

impl Default for Session
{
    fn default() -> Self {
        Session::Pending(Pending::PendingWelcome)
    }
}
