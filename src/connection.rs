use mio::tcp::TcpStream;
use mio;

/// An FTP connection
pub struct Connection
{
    pub pi: Interpreter,
    pub dtp: Option<DataTransfer>,
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
pub struct DataTransfer
{
    /// The underlying socket.
    pub stream: TcpStream,
    /// The token used to listen for events on the DTP stream.
    pub token: mio::Token,
}

impl Connection
{
    pub fn has_token(&self, token: mio::Token) -> bool {
        if self.pi.token == token { return true };

        if let Some(dtp) = self.dtp.as_ref() {
            if dtp.token == token { return true };
        }

        false
    }
}
