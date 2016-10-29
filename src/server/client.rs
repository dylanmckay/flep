use Connection;
use server::ClientState;
use protocol;

use std::io::prelude::*;
use std::io;

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
    pub fn receive_data(&mut self, token: mio::Token) -> Result<(), io::Error> {
        let mut buffer: [u8; 10000] = [0; 10000];
        if token == self.connection.pi.token {
            let bytes_written = self.connection.pi.stream.read(&mut buffer)?;
            let mut data = io::Cursor::new(&buffer[0..bytes_written]);

            let message = protocol::CommandKind::read(&mut data)?;

            println!("receiving data on PI stream: {:?}", message);
        } else {
            let dtp = self.connection.dtp.as_mut().unwrap();
            assert_eq!(dtp.token, token);

            let bytes_written = dtp.stream.read(&mut buffer)?;
            let data = &buffer[0..bytes_written];

            println!("receiving data on DTP stream: {:?}", data);
        }

        Ok(())
    }
}
