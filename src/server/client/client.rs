use {Error, protocol};
use io::{Connection, DataTransfer, DataTransferMode, Io};
use server::Server;
use server::client::{ClientState, Session};

use std::io::prelude::*;
use std;

use mio::unix::UnixReady;
use mio;

/// A client from the perspective of a server.
pub struct Client
{
    /// The current state of the client.
    pub state: ClientState,
    /// The network connection to the client.
    pub connection: Connection,
}

impl Client
{
    /// Attempts to update the state of the client with any
    /// information received from the network.
    pub fn tick(&mut self, io: &mut Io) -> Result<(), Error> {
        self::tick(&mut self.state, &mut self.connection, io)
    }

    pub fn handle_io_event(&mut self,
                           event: &mio::Event,
                           the_token: mio::Token,
                           server: &mut Server,
                           io: &mut Io)
        -> Result<(), Error> {
        super::client_io::handle_event(&mut self.state, event,
                                       &mut self.connection, the_token,
                                       server, io)
    }
}

/// Does the state tick.
fn tick(state: &mut ClientState,
        connection: &mut Connection,
        io: &mut Io) -> Result<(), Error> {
    match state.session {
        Session::Ready(ref mut session) => {
            let active_transfer = std::mem::replace(&mut session.active_transfer, None);

            if let Some(active_transfer) = active_transfer {
                let dtp = std::mem::replace(&mut connection.dtp, DataTransfer::None);

                debug!("server is ready and we have an active transfer");
                connection.dtp = match dtp {
                    DataTransfer::None => {
                        assert_eq!(session.data_transfer_mode, DataTransferMode::Active);

                        let client_addr = session.client_addr.expect("attempted a transfer but client address is not set");
                        let stream = mio::tcp::TcpStream::connect(&client_addr)?;

                        let token = io.allocate_token();
                        io.poll.register(&stream, token,
                                         mio::Ready::readable() | UnixReady::hup() |
                                         mio::Ready::writable(),
                                         mio::PollOpt::edge())?;

                        debug!("establishing a DTP connection for ACTIVE mode");

                        // We aren't ready to send data just yet.
                        session.active_transfer = Some(active_transfer);

                        DataTransfer::Connecting {
                            stream: stream,
                            token: token,
                        }
                    },
                    DataTransfer::Connected { mut stream, .. } => {
                        debug!("DTP stream is connected, sending data");

                        connection.send_command(&protocol::command::TYPE {
                            file_type: active_transfer.file_type,
                        })?;

                        stream.write(&active_transfer.data)?;
                        stream.flush()?;
                        drop(stream);

                        connection.send_reply(protocol::Reply::new(226, "Transfer complete"))?;

                        debug!("completed active transfer");
                        DataTransfer::None
                    },
                    state => {
                        // We aren't ready to send data just yet.
                        session.active_transfer = Some(active_transfer);
                        state
                    },
                };
            }

            Ok(())
        },
        _ => Ok(())
    }
}
