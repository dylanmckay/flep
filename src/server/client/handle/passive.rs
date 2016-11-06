use {Error, DataTransferMode, DataTransfer, Io, server, protocol};

pub fn handle_pasv(client: &mut server::Client,
                   io: &mut Io)
    -> Result<protocol::Reply, Error> {
    let port = listen_passive_dtp(client, io)?;
    Ok(protocol::reply::pasv::success(port))
}

pub fn handle_epsv(client: &mut server::Client,
                   io: &mut Io)
    -> Result<protocol::Reply, Error> {
    let port = listen_passive_dtp(client, io)?;
    Ok(protocol::reply::epsv::success(port))
}

/// Attempts to open a data connection passively.
fn listen_passive_dtp(client: &mut server::Client, io: &mut Io)
    -> Result<u16, Error> {
    let mut session = client.session.expect_ready_mut()?;
    let port = 5166;

    session.data_transfer_mode = DataTransferMode::Passive { port: port };
    client.connection.dtp = DataTransfer::bind(port, io).unwrap();
    Ok(port)
}
