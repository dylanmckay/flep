use {Reply, reply};

/// Forms a sucessful reply to a 'CDUP' command.
pub fn success() -> Reply {
    Reply::new(reply::code::REQUESTED_FILE_ACTION_COMPLETED,
               "changed to parent directory")
}
