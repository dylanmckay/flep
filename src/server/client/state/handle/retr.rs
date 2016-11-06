use {Error, FileType, server, protocol};
use std::path::Path;

/// Handle the 'RETR' command.
pub fn handle(retr: &protocol::RETR,
              client: &mut server::ClientState,
              ftp: &mut server::FileTransferProtocol)
    -> Result<protocol::Reply, Error> {
    client.session.expect_ready()?;

    let data = ftp.file_system().read(&Path::new(&retr.remote_filename))?;

    Ok(client.initiate_transfer(server::Transfer {
        file_type: FileType::ascii(),
        data: data,
    }))
}
