use Credentials;

/// An FTP server instance.
pub trait FileTransferProtocol : FileSystem
{
    /// Gets the welcome message shown when connecting to the server.
    fn welcome_message(&self) -> String;

    /// Attempts to authenticate a user.
    fn authenticate_user(&self, credentials: &Credentials) -> bool { true }
}

/// A filesystem mountable as FTP.
pub trait FileSystem
{
    // fn list(path) -> Vec<String> etc..
}
