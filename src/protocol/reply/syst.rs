use {Reply, reply};

/// Generates a successful response to the 'SYST' command.
///
/// * `os` is the operating system. It should be one of the
/// assigned constants from RFC 943.
pub fn successful(os: String) -> Reply {
    Reply::new(reply::code::SYSTEM_NAME_TYPE, os)
}
