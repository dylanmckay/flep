use Connection;
use server::ClientState;
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
    pub state: ClientState,
    pub connection: Connection,
}

impl Client
{
    pub fn new(stream: mio::tcp::TcpStream, token: mio::Token) -> Self {
        Client {
            uuid: Uuid::new_v4(),
            state: Default::default(),
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

        match command {
            // User attempting to log in.
            USER(ref user) => {
                if let ClientState::WaitingForLogin = self.state {
                    protocol::Reply::new(protocol::reply::code::USER_LOGGED_IN, "user logged in")
                } else {
                    panic!("received USER message at incorrect time");
                }
            },
            // Client requesting information about the server system.
            SYST(ref syst) => {
                protocol::Reply::new(protocol::reply::code::SYSTEM_NAME_TYPE, protocol::rfc1700::system::UNIX)
            },
            c => panic!("don't know how to handle {:?}", c),
        }
    }

    /// Attempts to progress the state of the client if need be.
    pub fn progress(&mut self, ftp: &mut server::FileTransferProtocol)
        -> Result<(), server::Error> {
        let state = std::mem::replace(&mut self.state, ClientState::WaitingForLogin);

        self.state = match state {
            ClientState::PendingWelcome => {
                println!("sending welcome");
                let welcome = protocol::Reply::new(protocol::reply::code::OK, ftp.welcome_message());
                welcome.write(&mut self.connection.pi.stream).unwrap();

                ClientState::WaitingForLogin
            },
            state => state,
        };

        Ok(())
    }
}
