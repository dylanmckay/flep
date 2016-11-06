use {Error, server, protocol};
use server::client::Action;
use std::path::Path;

/// Handle the 'MKD' command.
pub fn handle(mkd: &protocol::MKD,
              client: &mut server::ClientState,
              ftp: &mut server::FileTransferProtocol)
-> Result<Action, Error> {
    let session = client.session.expect_ready()?;

    let path = Path::new(&mkd.remote_filename);

    let path = if path.has_root() {
        path.to_owned()
    } else {
        session.working_dir.join(path)
    };

    let parent = path.parent().unwrap();
    let folder_name = path.file_name().unwrap().to_str().unwrap().to_owned();

    ftp.file_system_mut().mkdir(&parent, folder_name)?;

    Ok(Action::Reply(protocol::Reply::new(protocol::reply::code::PATHNAME_CREATED,
                     "created directory")))
}
