pub use self::client::Client;
pub use self::client_state::ClientState;

pub mod client;
pub mod client_state;

use Connection;
use connection;

use std::collections::{HashMap, hash_map};

use mio::*;
use mio::tcp::TcpListener;

use uuid::Uuid;

// Setup some tokens to allow us to identify which event is
// for which socket.
const SERVER: Token = Token(0);

pub fn run() {
    let mut token_accumulator: usize = 1;

    let addr = "127.0.0.1:2222".parse().unwrap();

    // Setup the server socket
    let server = TcpListener::bind(&addr).unwrap();

    // Create an poll instance
    let poll = Poll::new().unwrap();

    // Start listening for incoming connections
    poll.register(&server, SERVER, Ready::readable(),
                  PollOpt::edge()).unwrap();

    // Create storage for events
    let mut events = Events::with_capacity(1024);

    let mut clients = HashMap::new();

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    let (mut sock, _) = server.accept().unwrap();

                    // Increase the token accumulator so the connection gets a unique token.
                    token_accumulator += 1;

                    println!("accepted connection");

                    let token = Token(token_accumulator);
                    poll.register(&sock, token, Ready::readable() | Ready::hup(),
                                  PollOpt::edge()).unwrap();

                    let uuid = Uuid::new_v4();

                    let welcome = ::protocol::raw::Reply::new(200, "Hello There");
                    welcome.write(&mut sock).unwrap();

                    clients.insert(uuid, Client {
                        uuid: uuid,
                        state: Default::default(),
                        connection: Connection {
                            pi: connection::Interpreter {
                                stream: sock,
                                token: token,
                            },
                            dtp: None,
                        },
                    });
                }
                token => {
                    let client_uuid = clients.values().find(|client| client.connection.has_token(token)).unwrap().uuid;
                    let mut client = if let hash_map::Entry::Occupied(entry) = clients.entry(client_uuid) { entry } else { unreachable!() };

                    if event.kind().is_readable() {
                        client.get_mut().receive_data(token).unwrap();
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
