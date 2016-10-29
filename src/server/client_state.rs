/// The state of a client.
pub enum ClientState
{
    /// We need to send them a welcome message.
    PendingWelcome,
    /// The client needs to login.
    WaitingForLogin,
}

impl Default for ClientState
{
    fn default() -> Self { ClientState::PendingWelcome }
}
