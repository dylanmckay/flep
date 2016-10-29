/// An FTP server instance.
pub trait FileTransferProtocol : FileSystem
{
    fn welcome_message(&self) -> String;
}

/// A filesystem mountable as FTP.
pub trait FileSystem
{
    // fn list(path) -> Vec<String> etc..
}
