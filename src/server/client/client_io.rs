use {Connection, DataTransfer, Error, Io};
use server::client::Action;
use server::ClientState;
use {server, protocol};

use std::io::prelude::*;
use std::io;
use std;

use mio;

/// Handles an IO event on the protocol or data connections.
pub fn handle_event(state: &mut ClientState,
                    event: &mio::Event,
                    connection: &mut Connection,
                    the_token: mio::Token,
                    ftp: &mut server::FileTransferProtocol,
                    io: &mut Io)
    -> Result<(), Error> {
    if the_token == connection.pi.token && event.kind().is_readable() {
        handle_protocol_event(state, event, connection, ftp)
    } else {
        handle_data_event(event, connection, io)
    }
}

/// Handles an IO event on the protocol stream.
fn handle_protocol_event(state: &mut ClientState,
                         event: &mio::Event,
                         connection: &mut Connection,
                         ftp: &mut server::FileTransferProtocol)
    -> Result<(), Error> {
    let mut buffer: [u8; 10000] = [0; 10000];
    let bytes_written = connection.pi.stream.read(&mut buffer)?;
    let mut data = io::Cursor::new(&buffer[0..bytes_written]);

    assert_eq!(event.kind().is_readable(), true);

    if !data.get_ref().is_empty() {
        let command = protocol::CommandKind::read(&mut data)?;
        let action = match state.handle_command(&command, ftp) {
            Ok(action) => action,
            Err(e) => match e {
                // If it was state error, tell them.
                Error::Protocol(protocol::Error::Client(e)) => {
                    println!("error from state: {}", e.message());
                    Action::Reply(protocol::Reply::new(e.reply_code(), format!("error: {}", e.message())))
                },
                e => return Err(e),
            },
        };

        println!("action: {:?}", action);
        match action {
            Action::Reply(reply) => {
                reply.write(&mut connection.pi.stream)?;
            },
            Action::Transfer(transfer) => {
                let mut session = state.session.expect_ready_mut().unwrap();
                session.active_transfer = Some(transfer);
                unimplemented!();
            },
        }
    }

    Ok(())
}

/// Handles an IO event on the data stream.
fn handle_data_event(event: &mio::Event,
                     connection: &mut Connection,
                     io: &mut Io)
    -> Result<(), Error> {
    if event.kind().is_writable() {
        let dtp = std::mem::replace(&mut connection.dtp,
                                    DataTransfer::None);

        connection.dtp = match dtp {
            DataTransfer::None => unreachable!(),
            DataTransfer::Listening { listener, .. } => {
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

    Ok(())
}

