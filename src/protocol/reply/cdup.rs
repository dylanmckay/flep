use {Reply, reply};

/// Forms a sucessful reply to a 'CDUP' command.
pub fn success() -> Reply {
    Reply::new(reply::code::REQUESTED_FILE_ACTION_COMPLETED,
               "changed to parent directory")
}

/// Forms a failure reply to a 'CDUP' command in the case
/// where there is no parent directory.
pub fn no_parent() -> Reply {
    Reply::new(reply::code::REQUESTED_ACTION_NOT_TAKEN,
               "there is no parent directory")
}
