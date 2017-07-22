use {Error, ErrorKind, protocol};
use server::Server;
use server::client::{ClientState, Action};

use std::path::Path;

/// Handle the 'MKD' command.
pub fn handle(mkd: &protocol::MKD,
              client: &mut ClientState,
              server: &mut Server)
-> Result<Action, Error> {
    let session = client.session.expect_ready()?;

    let path = Path::new(&mkd.remote_filename);

    let path = if path.has_root() {
        path.to_owned()
    } else {
        session.working_dir.join(path)
    };

    match server.file_system_mut().create_dir(&path) {
        Ok(..) => Ok(Action::Reply(protocol::reply::mkd::success())),
        // IO errors are caused by the client, not us.
        Err(Error(ErrorKind::Io(..), _)) => {
            unimplemented!();
        },
        Err(e) => Err(e),
    }
}

