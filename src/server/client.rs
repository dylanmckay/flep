use {Connection, Credentials, DataTransfer, DataTransferMode, Error, Io};
use server::{Session, session};
use {server, protocol, connection};

use std::net::ToSocketAddrs;
use std::io::prelude::*;
use std::io;
use std;

use mio;
use uuid::Uuid;

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
        -> Result<(), server::Error> {
        let mut buffer: [u8; 10000] = [0; 10000];
        if the_token == self.connection.pi.token && event.kind().is_readable() {
            let bytes_written = self.connection.pi.stream.read(&mut buffer)?;
            let mut data = io::Cursor::new(&buffer[0..bytes_written]);

            let command = protocol::CommandKind::read(&mut data)?;
            let reply = self.handle_command(command, ftp, io);

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
                panic!("incoming data on DTP connection!");
            }
        }

        Ok(())
    }

    fn handle_command(&mut self,
                      command: protocol::CommandKind,
                      ftp: &mut server::FileTransferProtocol,
                      io: &mut Io) -> protocol::Reply {
        use protocol::CommandKind::*;

        println!("received {:?}", command);

        match command {
            // User attempting to log in.
            USER(ref user) => {
                if let Session::Login(session::Login::WaitingForUsername) = self.session {
                    let credentials = Credentials { username: user.username.to_owned(), password: None };

                    // The user may authenticate with no password
                    if ftp.authenticate_user(&credentials) {
                        self.session = Session::Ready(session::Ready::new(credentials));
                        protocol::Reply::new(protocol::reply::code::USER_LOGGED_IN, "user logged in")
                    } else {
                        // The user needs a password to get through.
                        self.session = Session::Login(session::Login::WaitingForPassword {
                            username: user.username.to_owned(),
                        });

                        protocol::Reply::new(protocol::reply::code::USER_NAME_OKAY_NEED_PASSWORD, "need password")
                    }
                } else {
                    // We can only handle USER commands during initialisation as of current
                    unimplemented!();
                }
            },
            PASS(ref pass) => {
                if let Session::Login(session::Login::WaitingForPassword { username }) = self.session.clone() {
                    let credentials = Credentials { username: username.to_owned(), password: Some(pass.password.to_owned()) };

                    if ftp.authenticate_user(&credentials) {
                        self.session = Session::Ready(session::Ready::new(credentials));
                        protocol::Reply::new(protocol::reply::code::USER_LOGGED_IN, "user logged in")
                    } else {
                        protocol::Reply::new(protocol::reply::code::USER_NOT_LOGGED_IN, "invalid credentials")
                    }
                } else {
                    panic!("username must be sent before password");
                }
            },
            PWD(..) => {
                if let Session::Ready(ref session) = self.session {
                    protocol::Reply::new(protocol::reply::code::PATHNAME_CREATED,
                                         session.working_dir.clone().into_os_string().into_string().unwrap())
                } else {
                    protocol::Reply::new(protocol::reply::code::USER_NOT_LOGGED_IN, "you must be logged in to do this")
                }
            },
            LIST(..) => {
                self.initiate_transfer(server::Transfer {
                    data: "foo".as_bytes().to_owned(),
                }).unwrap();

                if let DataTransfer::Connected { .. } = self.connection.dtp {
                    protocol::Reply::new(125, "transfer starting")
                } else {
                    protocol::Reply::new(150, "about to open data connection")
                }
            },
            // Client requesting information about the server system.
            SYST(..) => {
                protocol::Reply::new(protocol::reply::code::SYSTEM_NAME_TYPE, protocol::rfc1700::system::UNIX)
            },
            FEAT(..) => {
                protocol::response::feat::Features::default().into()
            },
            TYPE(ref ty) => {
                if let Session::Ready(ref mut session) = self.session {
                    session.transfer_type = ty.file_type;

                    println!("file type set to {:?}", ty.file_type);
                    protocol::Reply::new(protocol::reply::code::OK, "file type set")
                } else {
                    panic!("send TYPE command too early, need to be logged in first");
                }
            },
            PASV(..) => {
                if let Session::Ready(ref mut session) = self.session {
                    let port = 2223;
                    session.data_connection_mode = DataTransferMode::Passive { port: port };
                    self.connection.dtp = DataTransfer::bind(port, io).unwrap();

                    let port_bytes = [(port & 0xff00) >> 8,
                                      (port & 0x00ff) >> 0];
                    let textual_port = format!("{},{}", port_bytes[0], port_bytes[1]);

                    println!("passive mode enabled");
                    protocol::Reply::new(protocol::reply::code::ENTERING_PASSIVE_MODE,
                                         format!("passive mode enabled (127,0,0,1,{})", textual_port))
                } else {
                    panic!("send PASV command too early, need to be logged in first");
                }
            },
            PORT(ref port) => {
                println!("set port to {}", port.port);

                if let Session::Ready(ref mut session) = self.session {
                    session.port = Some(port.port);
                    protocol::Reply::new(protocol::reply::code::OK, "port")
                } else {
                    panic!("send PORT command too early, need to be logged in first");
                }
            },
            c => panic!("don't know how to handle {:?}", c),
        }
    }

    /// Attempts to progress the state of the client if need be.
    pub fn progress(&mut self, ftp: &mut server::FileTransferProtocol)
        -> Result<(), server::Error> {
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
            if let DataTransferMode::Passive { .. } = session.data_connection_mode {
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
                    match self.connection.dtp {
                        DataTransfer::None => {
                            assert_eq!(session.data_connection_mode, DataTransferMode::Active);

                            let addr = ("127.0.0.1", session.port.unwrap()).to_socket_addrs()?.next().unwrap();
                            let stream = mio::tcp::TcpStream::connect(&addr)?;

                            let token = io.allocate_token();
                            io.poll.register(&stream, token,
                                             mio::Ready::readable() | mio::Ready::hup() |
                                             mio::Ready::writable(),
                                             mio::PollOpt::edge())?;

                            self.connection.dtp = DataTransfer::Connecting {
                                stream: stream,
                                token: token,
                            };

                            println!("Establishing a DTP connection for ACTIVE mode");

                            // We aren't ready to send data just yet.
                            session.active_transfer = Some(active_transfer);
                        },
                        DataTransfer::Listening { .. } |
                            DataTransfer::Connecting { .. } => {
                            // We aren't ready to send data just yet.
                            session.active_transfer = Some(active_transfer);
                        },
                        DataTransfer::Connected { ref mut stream, .. } => {
                            stream.write(&active_transfer.data)?;
                        },
                    }
                }

                Ok(())
            },
            _ => Ok(())
        }
    }

    fn initiate_transfer(&mut self, transfer: server::Transfer) -> Result<(), Error> {
        if let Session::Ready(ref mut session) = self.session {
            assert_eq!(session.active_transfer, None);
            session.active_transfer = Some(transfer);
            Ok(())
        } else {
            panic!("in the middle of a transfer");
        }
    }
}
