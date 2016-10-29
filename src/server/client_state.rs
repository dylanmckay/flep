/// The state of a client.
pub enum ClientState
{
    /// We need to send them a welcome message.
    PendingWelcome,
}

impl Default for ClientState
{
    fn default() -> Self { ClientState::PendingWelcome }
}
