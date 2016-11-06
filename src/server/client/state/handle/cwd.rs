use {Error, server, protocol};

/// Handle the 'CWD' command.
pub fn handle(cwd: &protocol::CWD,
              client: &mut server::ClientState) -> Result<protocol::Reply, Error> {
    let mut session = client.session.expect_ready_mut()?;

    session.working_dir = cwd.path.clone().into();
    Ok(protocol::reply::cwd::success())
}
