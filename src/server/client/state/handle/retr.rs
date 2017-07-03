use {Error, FileType, server, protocol};
use server::client::{ClientState, Action};
use std::path::Path;

/// Handle the 'RETR' command.
pub fn handle(retr: &protocol::RETR,
              client: &mut ClientState,
              ftp: &mut server::FileTransferProtocol)
    -> Result<Action, Error> {
    client.session.expect_ready()?;

    let data = ftp.file_system().read(&Path::new(&retr.remote_filename))?;

    Ok(Action::Transfer(server::Transfer {
        file_type: FileType::ascii(),
        data: data,
    }))
}
