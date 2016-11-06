use {Error, protocol};

/// Handle the 'SYST' command.
pub fn handle() -> Result<protocol::Reply, Error> {
    Ok(protocol::reply::syst::successful(protocol::rfc1700::system::UNIX.to_owned()))
}
