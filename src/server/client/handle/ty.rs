use {Error, server, protocol};

/// Handle the 'TYPE' command.
pub fn handle(ty: &protocol::TYPE,
              client: &mut server::Client) -> Result<protocol::Reply, Error> {
    let mut session = client.session.expect_ready_mut()?;

    session.transfer_type = ty.file_type;

    println!("file type set to {:?}", ty.file_type);
    Ok(protocol::Reply::new(protocol::reply::code::OK, "file type set"))
}
