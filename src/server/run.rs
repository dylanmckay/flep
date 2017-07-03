//! The main server loop.

use Error;
use server::Server;
use server::client::{Client, ClientState};
use io::{Connection, Io, Interpreter, DataTransfer};

use uuid::Uuid;
use mio::unix::UnixReady;
use mio::tcp::TcpListener;
use mio::*;

use std::collections::{HashMap, hash_map};
use std::time::Duration;
use std::net::ToSocketAddrs;

/// The mio token used for the server connection.
const SERVER_TOKEN: Token = Token(0);

/// The state of an FTP server.
struct ServerState
{
    pub clients: HashMap<Uuid, Client>,
}

/// Runs a FTP server on a given address.
///
/// Sets up an FTP server locally and begins to wait for clients
/// to connect.
pub fn run<F,A>(server: &mut F, address: A) -> Result<(), Error>
    where F: Server,
          A: ToSocketAddrs {
    let mut addresses = address.to_socket_addrs()?;
    let address = match addresses.next() {
        Some(addr) => addr,
        None => return Err("could not resolve to any addresses".into()),
    };

    debug!("running server");

    // Setup the server socket
    let listener = TcpListener::bind(&address)?;
    let mut io = Io::new()?;

    // Start listening for incoming connections
    io.poll.register(&listener, SERVER_TOKEN, Ready::readable(),
                     PollOpt::edge())?;

    // Create storage for events
    let mut events = Events::with_capacity(1024);
    let mut state = ServerState::new();

    loop {
        for client_data in state.clients.values_mut() {
            client_data.tick(&mut io)?;
        }

        io.poll.poll(&mut events, Some(Duration::from_millis(30)))?;

        'events: for event in events.iter() {
            let readiness = UnixReady::from(event.readiness());

            match event.token() {
                SERVER_TOKEN => {
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    let (sock, _) = listener.accept()?;

                    // Increase the token accumulator so the connection gets a unique token.
                    let token = io.allocate_token();
                    io.poll.register(&sock, token, Ready::readable() | UnixReady::hup(),
                                  PollOpt::edge())?;

                    let mut client_state = ClientState::new();

                    let mut connection = Connection {
                        pi: Interpreter {
                            stream: sock,
                            token: token,
                        },
                        dtp: DataTransfer::None,
                    };

                    match client_state.progress(server, &mut connection) {
                        Ok(..) => {
                            debug!("a client has connected ({})", client_state.uuid);
                            state.clients.insert(client_state.uuid.clone(), Client {
                                state: client_state,
                                connection: connection,
                            });
                        },
                        Err(e) => {
                            info!("error while progressing client: {:?}", e);
                            drop(client_state);
                        }
                    }

                },
                token => {
                    let client_uuid = state.clients.values().find(|client| client.connection.uses_token(token)).unwrap().state.uuid;
                    let mut client = if let hash_map::Entry::Occupied(entry) = state.clients.entry(client_uuid) { entry } else { unreachable!() };

                    let mut should_remove = false;

                    {
                        let mut client_data = client.get_mut();
                        if let Err(e) = client_data.handle_io_event(&event, token, server, &mut io) {
                            info!("error while processing data from client ({}): {:?}", client_data.state.uuid, e);
                            should_remove = true;
                        }
                    }

                    if should_remove {
                        client.remove();
                        continue 'events;
                    }

                    if readiness.is_hup() {
                        info!("client disconnected");
                        client.remove();
                    }
                }
            }
        }
    }
}

impl ServerState
{
    /// Creates a new FTP server.
    pub fn new() -> Self {
        ServerState { clients: HashMap::new() }
    }
}

