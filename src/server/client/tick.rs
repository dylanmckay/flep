use {Connection, DataTransfer, DataTransferMode, Error, Io};
use server::client::{Client, Session};
use protocol;

use std::net::ToSocketAddrs;
use std::io::prelude::*;
use std;

use mio;

/// Does the client tick.
pub fn tick(client: &mut Client,
            connection: &mut Connection,
            io: &mut Io) -> Result<(), Error> {
    match client.session {
        Session::Ready(ref mut session) => {
            let active_transfer = std::mem::replace(&mut session.active_transfer, None);

            if let Some(active_transfer) = active_transfer {
                let dtp = std::mem::replace(&mut connection.dtp, DataTransfer::None);

                connection.dtp = match dtp {
                    DataTransfer::None => {
                        assert_eq!(session.data_transfer_mode, DataTransferMode::Active);

                        let addr = ("127.0.0.1", session.port.unwrap()).to_socket_addrs()?.next().unwrap();
                        let stream = mio::tcp::TcpStream::connect(&addr)?;

                        let token = io.allocate_token();
                        io.poll.register(&stream, token,
                                         mio::Ready::readable() | mio::Ready::hup() |
                                         mio::Ready::writable(),
                                         mio::PollOpt::edge())?;

                        println!("Establishing a DTP connection for ACTIVE mode");

                        // We aren't ready to send data just yet.
                        session.active_transfer = Some(active_transfer);

                        DataTransfer::Connecting {
                            stream: stream,
                            token: token,
                        }
                    },
                    DataTransfer::Connected { mut stream, .. } => {
                        println!("sent file");

                        connection.send_command(&protocol::command::TYPE {
                            file_type: active_transfer.file_type,
                        })?;

                        stream.write(&active_transfer.data)?;
                        stream.flush()?;
                        drop(stream);

                        std::thread::sleep(std::time::Duration::from_millis(800));
                        connection.send_reply(protocol::Reply::new(226, "Transfer complete"))?;

                        DataTransfer::None
                    },
                    state => {
                        // We aren't ready to send data just yet.
                        session.active_transfer = Some(active_transfer);
                        state
                    },
                };
            }

            Ok(())
        },
        _ => Ok(())
    }
}
