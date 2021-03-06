use {Error, FileType, server, protocol};
use server::Server;
use server::client::{ClientState, Action};
use std::path::Path;

/// Handle the 'RETR' command.
pub fn handle(retr: &protocol::RETR,
              client: &mut ClientState,
              server: &mut Server)
    -> Result<Action, Error> {
    client.session.expect_ready()?;

    let data = server.file_system().read_file(&Path::new(&retr.remote_filename))?;

    Ok(Action::Transfer(server::Transfer {
        file_type: FileType::ascii(),
        data: data,
    }))
}
