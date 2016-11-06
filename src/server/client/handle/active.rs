use {Error, server, protocol};

/// Handle the 'PORT' command.
pub fn handle_port(port: &protocol::PORT,
                   client: &mut server::Client)
    -> Result<protocol::Reply, Error> {
    let mut session = client.session.expect_ready_mut()?;

    println!("set port to {}", port.port);
    session.port = Some(port.port);
    Ok(protocol::Reply::new(protocol::reply::code::OK, "port"))
}
