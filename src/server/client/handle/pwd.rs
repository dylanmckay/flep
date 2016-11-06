use {Error, server, protocol};

/// Handle the 'PWD' command.
pub fn handle(client: &mut server::Client) -> Result<protocol::Reply, Error> {
    let session = client.session.expect_ready()?;
    Ok(protocol::reply::pwd::success(&session.working_dir))
}
