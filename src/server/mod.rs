pub use self::ftp::FileTransferProtocol;
pub use self::client::{Client, ClientState};
pub use self::transfer::Transfer;
pub use self::fs::FileSystem;
pub use self::server::Server;

pub mod ftp;
pub mod client;
pub mod transfer;
pub mod server;

pub mod fs;

use {Connection, Io};
use std::collections::hash_map;
use std::time::Duration;

use mio::*;
use mio::tcp::TcpListener;

const PROTOCOL_SERVER: Token = Token(0);

pub fn run<F>(mut ftp: F) where F: FileTransferProtocol {
    let protocol_addr = "127.0.0.1:2222".parse().unwrap();

    // Setup the server socket
    let protocol_server = TcpListener::bind(&protocol_addr).unwrap();

    let mut io = Io::new().unwrap();

    // Start listening for incoming connections
    io.poll.register(&protocol_server, PROTOCOL_SERVER, Ready::readable(),
                  PollOpt::edge()).unwrap();

    // Create storage for events
    let mut events = Events::with_capacity(1024);

    let mut server = Server::new();

    loop {
        for client_data in server.clients.values_mut() {
            client_data.tick(&mut io).unwrap();
        }

        io.poll.poll(&mut events, Some(Duration::from_millis(30))).unwrap();

        'events: for event in events.iter() {
            match event.token() {
                PROTOCOL_SERVER => {
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    let (sock, _) = protocol_server.accept().unwrap();

                    // Increase the token accumulator so the connection gets a unique token.
                    let token = io.allocate_token();
                    io.poll.register(&sock, token, Ready::readable() | Ready::hup(),
                                  PollOpt::edge()).unwrap();

                    let mut client_state = ClientState::new();

                    let mut connection = Connection {
                        pi: ::connection::Interpreter {
                            stream: sock,
                            token: token,
                        },
                        dtp: ::connection::DataTransfer::None,
                    };

                    match client_state.progress(&mut ftp, &mut connection) {
                        Ok(..) => {
                            println!("a client has connected ({})", client_state.uuid);
                            server.clients.insert(client_state.uuid.clone(), Client {
                                state: client_state,
                                connection: connection,
                            });
                        },
                        Err(e) => {
                            println!("error while progressing client: {:?}", e);
                            drop(client_state);
                        }
                    }

                },
                token => {
                    let client_uuid = server.clients.values().find(|client| client.connection.uses_token(token)).unwrap().state.uuid;
                    let mut client = if let hash_map::Entry::Occupied(entry) = server.clients.entry(client_uuid) { entry } else { unreachable!() };
                    println!("event: {:?}", event);

                    let mut should_remove = false;

                    {
                        let mut client_data = client.get_mut();
                        if let Err(e) = client_data.handle_io_event(&event, token, &mut ftp, &mut io) {
                            println!("error while processing data from client ({}): {:?}", client_data.state.uuid, e);
                            should_remove = true;
                        }
                    }

                    if should_remove {
                        client.remove();
                        continue 'events;
                    }

                    if event.kind().is_hup() {
                        println!("client disconnected");
                        client.remove();
                    }
                }
            }
        }
    }
}
