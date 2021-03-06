use Error;
use protocol;
use super::Io;

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
    Connecting {
        /// The underlying socket.
        stream: TcpStream,
        /// The token used to listen for events on the DTP stream.
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
pub enum DataTransferMode
{
    /// The server will attempt to initialize a connection on the client.
    Active,
    /// The client will make create the DTP connection and we will use it.
    Passive { port: u16 },
}

impl Connection
{
    pub fn send_command<C>(&mut self, command: &C) -> Result<(), Error>
        where C: protocol::Command {
        command.write(&mut self.pi.stream)?;
        Ok(())
    }

    pub fn send_reply<R>(&mut self, reply: R) -> Result<(), Error>
        where R: Into<protocol::Reply> {
        let reply = reply.into();
        reply.write(&mut self.pi.stream)?;
        Ok(())
    }

    pub fn uses_token(&self, the_token: mio::Token) -> bool {
        if self.pi.token == the_token { return true };

        match self.dtp {
            DataTransfer::None => false,
            DataTransfer::Listening { ref token, .. } => *token == the_token,
            DataTransfer::Connecting { ref token, .. } => *token == the_token,
            DataTransfer::Connected { ref token, .. } => *token == the_token,
        }
    }
}

impl DataTransfer
{
    /// Start listening for a new data transfer on a port.
    pub fn bind(port: u16, io: &mut Io) -> Result<Self, Error> {
        use std::net::ToSocketAddrs;

        let addr = ("127.0.0.1", port).to_socket_addrs()?.next().unwrap();
        let listener = TcpListener::bind(&addr)?;

        let token = io.allocate_token();

        io.poll.register(&listener, token, mio::Ready::readable(),
                      mio::PollOpt::edge())?;

        Ok(DataTransfer::Listening {
            listener: listener,
            token: token,
        })
    }

    pub fn connect() -> Result<Self, Error> {
        unimplemented!();
    }
}

impl Default for DataTransferMode
{
    // FTP defaults to active mode (unless you send 'PASV').
    fn default() -> Self { DataTransferMode::Active }
}
