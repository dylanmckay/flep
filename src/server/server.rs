use Error;
use server::FileTransferProtocol;
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

impl ServerState
{
    /// Creates a new FTP server.
    pub fn new() -> Self {
        ServerState { clients: HashMap::new() }
    }
}

pub fn run<F,A>(ftp: &mut F, address: A) -> Result<(), Error>
    where F: FileTransferProtocol,
          A: ToSocketAddrs {
    let mut addresses = address.to_socket_addrs()?;
    let address = match addresses.next() {
        Some(addr) => addr,
        None => return Err("could not resolve to any addresses".into()),
    };

    // Setup the server socket
    let listener = TcpListener::bind(&address)?;
    let mut io = Io::new()?;

    // Start listening for incoming connections
    io.poll.register(&listener, SERVER_TOKEN, Ready::readable(),
                     PollOpt::edge())?;

    // Create storage for events
    let mut events = Events::with_capacity(1024);
    let mut server = ServerState::new();

    loop {
        for client_data in server.clients.values_mut() {
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

                    match client_state.progress(ftp, &mut connection) {
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
                        if let Err(e) = client_data.handle_io_event(&event, token, ftp, &mut io) {
                            println!("error while processing data from client ({}): {:?}", client_data.state.uuid, e);
                            should_remove = true;
                        }
                    }

                    if should_remove {
                        client.remove();
                        continue 'events;
                    }

                    if readiness.is_hup() {
                        println!("client disconnected");
                        client.remove();
                    }
                }
            }
        }
    }
}
