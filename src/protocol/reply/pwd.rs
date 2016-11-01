use {Reply, reply};
use std::path::Path;

/// Forms a sucessful reply to a 'PWD' command.
pub fn success(working_dir: &Path) -> Reply {
    let escaped_dir = format!("\"{}\"",
                              working_dir.to_owned().into_os_string().into_string().unwrap());
    // It's pretty weird that 'PWD' returns 'PATHNAME_CREATED'.
    // Here's what RFC 959 has to say:
    //
    // "Essentially because the PWD command returns the same type of
    // information as the successful MKD command, the successful PWD
    // command uses the 257 reply code as well."
    Reply::new(reply::code::PATHNAME_CREATED, escaped_dir)
}
