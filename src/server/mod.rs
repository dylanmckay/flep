pub use self::ftp::FileTransferProtocol;
pub use self::client::Client;
pub use self::session::Session;
pub use self::transfer::Transfer;
pub use self::fs::FileSystem;

pub mod ftp;
pub mod client;
pub mod session;
pub mod transfer;

pub mod fs;

use Io;
use std::collections::{HashMap, hash_map};
use std::time::Duration;

use mio::*;
use mio::tcp::TcpListener;

use uuid::Uuid;

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

    let mut clients: HashMap<Uuid, Client> = HashMap::new();

    loop {
        for client in clients.values_mut() {
            client.tick(&mut io).unwrap();
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
                token => {
                    let client_uuid = clients.values().find(|client| client.connection.uses_token(token)).unwrap().uuid;
                    let mut client = if let hash_map::Entry::Occupied(entry) = clients.entry(client_uuid) { entry } else { unreachable!() };
                    println!("event: {:?}", event);

                    if let Err(e) = client.get_mut().handle_io_event(&event, token, &mut ftp, &mut io) {
                        println!("error while processing data from client ({}): {:?}", client.get().uuid, e);
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
