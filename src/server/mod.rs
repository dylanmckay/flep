pub use self::ftp::{FileTransferProtocol, FileSystem};
pub use self::client::Client;
pub use self::session::Session;
pub use self::error::Error;

pub mod ftp;
pub mod client;
pub mod session;
pub mod error;

use std::collections::{HashMap, hash_map};

use mio::*;
use mio::tcp::TcpListener;

const PROTOCOL_SERVER: Token = Token(0);
const DATA_SERVER: Token = Token(1);

pub fn run<F>(mut ftp: F) where F: FileTransferProtocol {
    let mut token_accumulator: usize = 100;

    let protocol_addr = "127.0.0.1:2222".parse().unwrap();
    let data_addr = "127.0.0.1:2223".parse().unwrap();

    // Setup the server socket
    let protocol_server = TcpListener::bind(&protocol_addr).unwrap();
    let data_server = TcpListener::bind(&data_addr).unwrap();

    // Create an poll instance
    let poll = Poll::new().unwrap();

    // Start listening for incoming connections
    poll.register(&protocol_server, PROTOCOL_SERVER, Ready::readable(),
                  PollOpt::edge()).unwrap();

    poll.register(&data_server, DATA_SERVER, Ready::readable(),
                  PollOpt::edge()).unwrap();

    // Create storage for events
    let mut events = Events::with_capacity(1024);

    let mut clients = HashMap::new();

    loop {
        poll.poll(&mut events, None).unwrap();

        'events: for event in events.iter() {
            match event.token() {
                PROTOCOL_SERVER => {
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    let (sock, _) = protocol_server.accept().unwrap();

                    // Increase the token accumulator so the connection gets a unique token.
                    token_accumulator += 1;

                    let token = Token(token_accumulator);
                    poll.register(&sock, token, Ready::readable() | Ready::hup(),
                                  PollOpt::edge()).unwrap();

                    let mut client = Client::new(sock, token);

                    match client.progress(&mut ftp) {
                        Ok(..) => {
                            println!("a client has connected ({})", client.uuid);
                            clients.insert(client.uuid.clone(), client);
                        },
                        Err(e) => {
                            println!("error while progressing client: {:?}", e);
                            drop(client);
                        }
                    }

                },
                DATA_SERVER => {
                    panic!("received conn on the data server");
                },
                token => {
                    let client_uuid = clients.values().find(|client| client.connection.pi.token == token).unwrap().uuid;
                    let mut client = if let hash_map::Entry::Occupied(entry) = clients.entry(client_uuid) { entry } else { unreachable!() };

                    if event.kind().is_readable() {
                        if let Err(e) = client.get_mut().handle_data(token, &mut ftp) {
                            println!("error while processing data from client ({}): {:?}", client.get().uuid, e);
                            client.remove();
                            continue 'events;
                        }
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
