use {Error, DataTransferMode, server, protocol};

pub fn handle_pasv(client: &mut server::ClientState)
    -> Result<protocol::Reply, Error> {
    let port = listen_passive_dtp(client)?;
    Ok(protocol::reply::pasv::success(port))
}

pub fn handle_epsv(client: &mut server::ClientState)
    -> Result<protocol::Reply, Error> {
    let port = listen_passive_dtp(client)?;
    Ok(protocol::reply::epsv::success(port))
}

/// Attempts to open a data connection passively.
fn listen_passive_dtp(client: &mut server::ClientState)
    -> Result<u16, Error> {
    let mut session = client.session.expect_ready_mut()?;
    let port = 5166;

    session.data_transfer_mode = DataTransferMode::Passive { port: port };
    Ok(port)
}
