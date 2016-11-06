use {Error, server, protocol};

/// Handle the 'PWD' command.
pub fn handle(client: &mut server::ClientState) -> Result<protocol::Reply, Error> {
    let session = client.session.expect_ready()?;
    Ok(protocol::reply::pwd::success(&session.working_dir))
}
