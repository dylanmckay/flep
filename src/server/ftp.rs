use Credentials;
use fs::FileSystem;

/// An FTP server instance.
pub trait FileTransferProtocol
{
    /// Gets the welcome message shown when connecting to the server.
    fn welcome_message(&self) -> String;

    /// Attempts to authenticate a user.
    fn authenticate_user(&self, _credentials: &Credentials) -> bool { true }

    fn file_system(&self) -> &FileSystem;
    fn file_system_mut(&mut self) -> &mut FileSystem;
}
