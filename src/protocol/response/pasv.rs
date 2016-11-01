use {Reply, reply};

/// Forms a successful reply to a 'PASV' command.
pub fn success(port: u16) -> Reply {
    let port_bytes = [(port & 0xff00) >> 8,
                      (port & 0x00ff) >> 0];
    let textual_port = format!("{},{}", port_bytes[0], port_bytes[1]);

    Reply::new(
        reply::code::ENTERING_PASSIVE_MODE,
        format!("passive mode enabled (127,0,0,1,{})", textual_port))
}
