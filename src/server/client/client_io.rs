use {Connection, DataTransfer, Error, Io};
use server::Client;
use {server, protocol};

use std::io::prelude::*;
use std::io;
use std;

use mio;

pub fn handle_event(client: &mut Client,
                    event: &mio::Event,
                    connection: &mut Connection,
                    the_token: mio::Token,
                    ftp: &mut server::FileTransferProtocol,
                    io: &mut Io)
    -> Result<(), Error> {
    let mut buffer: [u8; 10000] = [0; 10000];
    if the_token == connection.pi.token && event.kind().is_readable() {
        let bytes_written = connection.pi.stream.read(&mut buffer)?;
        let mut data = io::Cursor::new(&buffer[0..bytes_written]);

        if !data.get_ref().is_empty() {
            let command = protocol::CommandKind::read(&mut data)?;
            let reply = match client.handle_command(&command, ftp) {
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

            reply.write(&mut connection.pi.stream)?;
        }
    } else {
        if event.kind().is_writable() {
            let dtp = std::mem::replace(&mut connection.dtp,
                                        DataTransfer::None);

            connection.dtp = match dtp {
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
            let dtp = std::mem::replace(&mut connection.dtp, DataTransfer::None);

            connection.dtp = match dtp {
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
