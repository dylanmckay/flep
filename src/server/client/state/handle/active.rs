use {Error, protocol};
use server::client::{ClientState, Action};

/// Handle the 'PORT' command.
pub fn handle_port(port: &protocol::PORT,
                   client: &mut ClientState)
    -> Result<Action, Error> {
    let mut session = client.session.expect_ready_mut()?;

    debug!("client requested we initiate an active DTP connection on port {}", port.port);

    // For active mode, we set the socket address on the session so that
    // we keep the address for later use. We do not have to worry in passive
    // mode because the client always initiates the data connection.
    session.client_addr = Some(port.to_socket_addr());
    Ok(Action::Reply(protocol::Reply::new(protocol::reply::code::OK, "port")))
}
