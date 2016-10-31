use mio::tcp::{TcpStream, TcpListener};
use mio;

/// An FTP connection
pub struct Connection
{
    pub pi: Interpreter,
    pub dtp: DataTransfer,
}

/// The protocol interpreter (PI) stream.
pub struct Interpreter
{
    /// The underlying socket.
    pub stream: TcpStream,
    /// The token used to listen for events on the PI stream.
    pub token: mio::Token,
}

/// The data transfer prototocol (DTP) stream.
pub enum DataTransfer
{
    /// No DTP stream has or is being set up.
    None,
    /// We are currently listening for the other end to open a data connection.
    Listening {
        /// The port we are listening on.
        listener: TcpListener,
        /// The token for the listener.
        token: mio::Token,
    },
    /// We are connected.
    Connected {
        /// The underlying socket.
        stream: TcpStream,
        /// The token used to listen for events on the DTP stream.
        token: mio::Token,
    },
}

/// How the data connection should be established.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DataConnectionMode
{
    /// The server will attempt to initialize a connection on the client.
    Active,
    /// The client will make create the DTP connection and we will use it.
    Passive,
}

impl Default for DataConnectionMode
{
    // FTP defaults to active mode (unless you send 'PASV').
    fn default() -> Self { DataConnectionMode::Active }
}
