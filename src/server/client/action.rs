use DataTransferMode;
use server::Transfer;
use protocol;

/// An action to take after receiving a command.
#[derive(Clone, Debug)]
pub enum Action
{
    /// Reply to the command normally.
    Reply(protocol::Reply),
    /// Establish a data connection.
    EstablishDataConnection {
        /// The reply to the original command.
        reply: protocol::Reply,
        /// The mode for the data connection.
        mode: DataTransferMode,
    },
    /// Transfer data.
    Transfer(Transfer),
}
