use {Connection, DataTransfer, DataTransferMode, Error, Io};
use server::{Session, session};
use {server, protocol, connection};

use std;

use mio;
use uuid::Uuid;

mod handle;
mod tick;
mod client_io;

/// An FTP client from the point-of-view of the FTP server.
pub struct Client
{
    pub uuid: Uuid,
    pub session: Session,
    pub connection: Connection,
}

impl Client
{
    pub fn new(stream: mio::tcp::TcpStream, token: mio::Token) -> Self {
        Client {
            uuid: Uuid::new_v4(),
            session: Default::default(),
            connection: Connection {
                pi: connection::Interpreter {
                    stream: stream,
                    token: token,
                },
                dtp: DataTransfer::None,
            },
        }
    }

    pub fn handle_io_event(&mut self, event: &mio::Event, the_token: mio::Token,
                           ftp: &mut server::FileTransferProtocol,
                           io: &mut Io)
        -> Result<(), Error> {
        self::client_io::handle_event(self, event, the_token, ftp, io)
    }

    fn handle_command(&mut self,
                      command: protocol::CommandKind,
                      ftp: &mut server::FileTransferProtocol,
                      io: &mut Io) -> Result<protocol::Reply, Error> {
        handle::command(self, command, ftp, io)
    }

    /// Attempts to progress the state of the client if need be.
    pub fn progress(&mut self, ftp: &mut server::FileTransferProtocol)
        -> Result<(), Error> {
        let session = std::mem::replace(&mut self.session, Session::default());

        self.session = match session {
            Session::PendingWelcome => {
                println!("sending welcome");
                let welcome = protocol::Reply::new(protocol::reply::code::OK, ftp.welcome_message());
                welcome.write(&mut self.connection.pi.stream).unwrap();

                Session::Login(session::Login::WaitingForUsername)
            },
            session => session,
        };

        Ok(())
    }

    /// Checks whether the client expects a connection on a given port.
    pub fn wants_connection_on_port(&self, port: u16) -> bool {
        if let Session::Ready(ref session) = self.session {
            if let DataTransferMode::Passive { .. } = session.data_transfer_mode {
                // We only expect incoming connections for this client if we're in
                // passive mode and have been told to expect a conn on this port.
                session.port == Some(port)
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn tick(&mut self,
                io: &mut Io) -> Result<(), Error> {
        tick::tick(self, io)
    }

    fn initiate_transfer(&mut self, transfer: server::Transfer)
        -> protocol::Reply {
        if let Session::Ready(ref mut session) = self.session {
            assert_eq!(session.active_transfer, None);
            session.active_transfer = Some(transfer);

            if let DataTransfer::Connected { .. } = self.connection.dtp {
                protocol::Reply::new(125, "transfer starting")
            } else {
                protocol::Reply::new(150, "about to open data connection")
            }
        } else {
            panic!("in the middle of a transfer");
        }
    }
}
