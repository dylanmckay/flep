use {Error, server, protocol};
use server::client::Action;

/// Handle the 'PORT' command.
pub fn handle_port(port: &protocol::PORT,
                   client: &mut server::ClientState)
    -> Result<Action, Error> {
    let mut session = client.session.expect_ready_mut()?;

    println!("set port to {}", port.port);
    session.port = Some(port.port);
    Ok(Action::Reply(protocol::Reply::new(protocol::reply::code::OK, "port")))
}
