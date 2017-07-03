//! Client state.
//!
//! This module should be free of *all* network IO.

pub use self::session::Session;

use {Error, server, protocol};
use io::Connection;

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
                welcome.write(&mut connection.pi.stream)?;

                Session::Login(session::Login::WaitingForUsername)
            },
            session => session,
        };

        Ok(())
    }
}
