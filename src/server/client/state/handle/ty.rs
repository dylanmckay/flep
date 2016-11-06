use {Error, server, protocol};
use server::client::Action;

/// Handle the 'TYPE' command.
pub fn handle(ty: &protocol::TYPE,
              client: &mut server::ClientState) -> Result<Action, Error> {
    let mut session = client.session.expect_ready_mut()?;

    session.transfer_type = ty.file_type;

    println!("file type set to {:?}", ty.file_type);
    Ok(Action::Reply(protocol::Reply::new(protocol::reply::code::OK, "file type set")))
}
