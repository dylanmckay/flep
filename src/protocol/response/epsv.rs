use {Reply, reply};

/// Forms a sucessful reply to a 'EPSV' command.
pub fn success(port: u16) -> Reply {
    Reply::new(
        reply::code::ENTERING_PASSIVE_MODE_EXTENDED,
        format!("passive mode enabled (|||{}|)", port))
}
