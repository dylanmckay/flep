use {Connection, Credentials};
use server::{Session, session};
use {server, protocol, connection};

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
                dtp: None,
            },
        }
    }

    pub fn handle_data(&mut self, token: mio::Token, ftp: &mut server::FileTransferProtocol)
        -> Result<(), server::Error> {
        let mut buffer: [u8; 10000] = [0; 10000];
        if token == self.connection.pi.token {
            let bytes_written = self.connection.pi.stream.read(&mut buffer)?;
            let mut data = io::Cursor::new(&buffer[0..bytes_written]);

            let command = protocol::CommandKind::read(&mut data)?;
            let reply = self.handle_command(command, ftp);

            reply.write(&mut self.connection.pi.stream)?;
        } else {
            let dtp = self.connection.dtp.as_mut().unwrap();
            assert_eq!(dtp.token, token);

            let bytes_written = dtp.stream.read(&mut buffer)?;
            let data = &buffer[0..bytes_written];

            println!("receiving data on DTP stream: {:?}", data);
        }

        Ok(())
    }

    fn handle_command(&mut self,
                      command: protocol::CommandKind,
                      ftp: &mut server::FileTransferProtocol) -> protocol::Reply {
        use protocol::CommandKind::*;

        println!("received {:?}", command);

        match command {
            // User attempting to log in.
            USER(ref user) => {
                if let Session::Pending(session::Pending::WaitingForUsername) = self.session {
                    let credentials = Credentials { username: user.username.to_owned(), password: None };

                    // The user may authenticate with no password
                    if ftp.authenticate_user(&credentials) {
                        self.session = Session::Ready(session::Ready::new(credentials));
                        protocol::Reply::new(protocol::reply::code::USER_LOGGED_IN, "user logged in")
                    } else {
                        // The user needs a password to get through.
                        self.session = Session::Pending(session::Pending::WaitingForPassword {
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
                if let Session::Pending(session::Pending::WaitingForPassword { username }) = self.session.clone() {
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
                println!("passive mode enabled");
                protocol::Reply::new(protocol::reply::code::ENTERING_PASSIVE_MODE, "passive mode enabled")
            },
            PORT(ref _port) => {
                protocol::Reply::new(protocol::reply::code::OK, "port")
            },
            c => panic!("don't know how to handle {:?}", c),
        }
    }

    /// Attempts to progress the state of the client if need be.
    pub fn progress(&mut self, ftp: &mut server::FileTransferProtocol)
        -> Result<(), server::Error> {
        let session = std::mem::replace(&mut self.session, Session::default());

        self.session = match session {
            Session::Pending(session::Pending::PendingWelcome) => {
                println!("sending welcome");
                let welcome = protocol::Reply::new(protocol::reply::code::OK, ftp.welcome_message());
                welcome.write(&mut self.connection.pi.stream).unwrap();

                Session::Pending(session::Pending::WaitingForUsername)
            },
            session => session,
        };

        Ok(())
    }
}
