use {Error, protocol};

/// Handle the 'FEAT' command.
pub fn handle() -> Result<protocol::Reply, Error> {
    Ok(protocol::reply::feat::Features::default().into())
}
