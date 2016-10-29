/// The state of a client.
pub enum ClientState
{
    AwaitingPort,
}

impl Default for ClientState
{
    fn default() -> Self { ClientState::AwaitingPort }
}
