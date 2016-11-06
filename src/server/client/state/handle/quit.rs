use {Error, protocol};

/// Handle the 'QUIT' command.
pub fn handle() -> Result<protocol::Reply, Error> {
    Ok(protocol::Reply::new(
        protocol::reply::code::SERVICE_CLOSING_CONTROL_CONNECTION,
        "goodbye"))
}
