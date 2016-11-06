//! Client state.
//!
//! This module should be free of *all* network IO.

pub use self::session::Session;

use {Connection, DataTransferMode, Error};
use {server, protocol};

use std;

use uuid::Uuid;

mod handle;
mod session;

/// An FTP client from the point-of-view of the FTP server.
pub struct ClientState
{
    pub uuid: Uuid,
    pub session: Session,
}

impl ClientState
{
    pub fn new() -> Self {
        ClientState {
            uuid: Uuid::new_v4(),
            session: Default::default(),
        }
    }

    pub fn handle_command(&mut self,
                      command: &protocol::CommandKind,
                      ftp: &mut server::FileTransferProtocol)
        -> Result<server::client::Action, Error> {
        handle::command(self, command, ftp)
    }

    /// Attempts to progress the state of the client if need be.
    pub fn progress(&mut self,
                    ftp: &mut server::FileTransferProtocol,
                    connection: &mut Connection)
        -> Result<(), Error> {
        let session = std::mem::replace(&mut self.session, Session::default());

        self.session = match session {
            Session::PendingWelcome => {
                println!("sending welcome");
                let welcome = protocol::Reply::new(protocol::reply::code::OK, ftp.welcome_message());
                welcome.write(&mut connection.pi.stream).unwrap();

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

    fn initiate_transfer(&mut self,
                         transfer: server::Transfer)
        -> protocol::Reply {
        if let Session::Ready(ref mut session) = self.session {
            assert_eq!(session.active_transfer, None);
            session.active_transfer = Some(transfer);

            unimplemented!();
            // if let DataTransfer::Connected { .. } = connection.dtp {
            //     protocol::Reply::new(125, "transfer starting")
            // } else {
            //     protocol::Reply::new(150, "about to open data connection")
            // }
        } else {
            panic!("in the middle of a transfer");
        }
    }
}
