use {Connection, DataTransfer, DataTransferMode, Error, Io};
use server::{Session, session};
use {server, protocol, connection};

use std::net::ToSocketAddrs;
use std::io::prelude::*;
use std::io;
use std;

use mio;
use uuid::Uuid;

mod handle_command;

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

    pub fn handle_event(&mut self,
                        event: &mio::Event,
                        the_token: mio::Token,
                        ftp: &mut server::FileTransferProtocol,
                        io: &mut Io)
        -> Result<(), Error> {
        let mut buffer: [u8; 10000] = [0; 10000];
        if the_token == self.connection.pi.token && event.kind().is_readable() {
            let bytes_written = self.connection.pi.stream.read(&mut buffer)?;
            let mut data = io::Cursor::new(&buffer[0..bytes_written]);

            let command = protocol::CommandKind::read(&mut data)?;
            let reply = match self.handle_command(command, ftp, io) {
                Ok(reply) => reply,
                Err(e) => match e {
                    // If it was client error, tell them.
                    Error::Protocol(protocol::Error::Client(e)) => {
                        println!("error from client: {}", e.message());
                        protocol::Reply::new(e.reply_code(), format!("error: {}", e.message()))
                    },
                    e => return Err(e),
                },
            };

            reply.write(&mut self.connection.pi.stream)?;
        } else {
            if event.kind().is_writable() {
                let dtp = std::mem::replace(&mut self.connection.dtp,
                                            DataTransfer::None);

                self.connection.dtp = match dtp {
                    DataTransfer::None => unreachable!(),
                    DataTransfer::Listening { listener, token } => {
                        assert_eq!(the_token, token);

                        let (sock, _) = listener.accept()?;

                        let connection_token = io.allocate_token();
                        io.poll.register(&sock, connection_token,
                                         mio::Ready::readable() | mio::Ready::hup(),
                                         mio::PollOpt::edge())?;

                        println!("data connection established via PASV mode");

                        DataTransfer::Connecting {
                            stream: sock,
                            token: connection_token,
                        }
                    },
                    DataTransfer::Connecting { stream, token } => {
                        println!("ACTIVE connection established");

                        // If we received an event on a connecting socket,
                        // it must be writable.
                        DataTransfer::Connected { stream: stream, token: token }
                    },
                    DataTransfer::Connected { stream, token } => {
                        assert_eq!(the_token, token);
                        DataTransfer::Connected { stream: stream, token: token }
                    },
                }
            }

            if event.kind().is_readable() {
                let dtp = std::mem::replace(&mut self.connection.dtp, DataTransfer::None);

                self.connection.dtp = match dtp {
                    DataTransfer::None => unreachable!(),
                    DataTransfer::Listening { listener, .. } => {
                        let (sock, _) = listener.accept()?;

                        let connection_token = io.allocate_token();
                        io.poll.register(&sock, connection_token,
                                         mio::Ready::readable() | mio::Ready::hup(),
                                         mio::PollOpt::edge())?;

                        DataTransfer::Connected {
                            stream: sock,
                            token: connection_token,
                        }
                    },
                    dtp => dtp,
                };
            }
        }

        Ok(())
    }

    fn handle_command(&mut self,
                      command: protocol::CommandKind,
                      ftp: &mut server::FileTransferProtocol,
                      io: &mut Io) -> Result<protocol::Reply, Error> {
        handle_command::handle(self, command, ftp, io)
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
        match self.session {
            Session::Ready(ref mut session) => {
                let active_transfer = std::mem::replace(&mut session.active_transfer, None);

                if let Some(active_transfer) = active_transfer {
                    let dtp = std::mem::replace(&mut self.connection.dtp, DataTransfer::None);

                    self.connection.dtp = match dtp {
                        DataTransfer::None => {
                            assert_eq!(session.data_transfer_mode, DataTransferMode::Active);

                            let addr = ("127.0.0.1", session.port.unwrap()).to_socket_addrs()?.next().unwrap();
                            let stream = mio::tcp::TcpStream::connect(&addr)?;

                            let token = io.allocate_token();
                            io.poll.register(&stream, token,
                                             mio::Ready::readable() | mio::Ready::hup() |
                                             mio::Ready::writable(),
                                             mio::PollOpt::edge())?;

                            println!("Establishing a DTP connection for ACTIVE mode");

                            // We aren't ready to send data just yet.
                            session.active_transfer = Some(active_transfer);

                            DataTransfer::Connecting {
                                stream: stream,
                                token: token,
                            }
                        },
                        DataTransfer::Connected { mut stream, .. } => {
                            println!("sent file");

                            self.connection.send_command(&protocol::command::TYPE {
                                file_type: active_transfer.file_type,
                            })?;

                            stream.write(&active_transfer.data)?;
                            stream.flush()?;
                            drop(stream);

                            std::thread::sleep(std::time::Duration::from_millis(800));
                            self.connection.send_reply(protocol::Reply::new(226, "Transfer complete"))?;

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
