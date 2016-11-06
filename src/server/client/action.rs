use protocol;
use server::Transfer;

/// An action to take after receiving a command.
#[derive(Clone, Debug)]
pub enum Action
{
    Reply(protocol::Reply),
    Transfer(Transfer),
}
