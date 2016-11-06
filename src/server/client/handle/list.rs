use {Error, FileType, server, protocol};

/// Handle the 'LIST' command.
pub fn handle(list: &protocol::LIST,
              client: &mut server::ClientState,
              ftp: &mut server::FileTransferProtocol)
    -> Result<protocol::Reply, Error> {
    if list.remote_filespec.is_some() {
        unimplemented!();
    }

    let working_dir = client.session.expect_ready()?.working_dir.clone();

    let entries = ftp.file_system().list(&working_dir)?;
    let mut data: String = entries.join("\r\n");
    data.extend("\r\n".chars());

    Ok(client.initiate_transfer(server::Transfer {
        file_type: FileType::ascii(),
        data: data.as_bytes().to_owned(),
    }))
}
