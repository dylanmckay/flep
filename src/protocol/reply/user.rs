use {Reply, reply};

/// Forms a reply telling the client that they have been successfully logged in.
pub fn logged_in() -> Reply {
    Reply::new(reply::code::USER_LOGGED_IN, "user logged in")
}

/// Forms a reply telling the client that we need the password before we
/// can complete authentication.
pub fn need_password() -> Reply {
    Reply::new(reply::code::USER_NAME_OKAY_NEED_PASSWORD, "need password")
}

/// Creates a reply telling the client that they were not logged in fore some reason.
pub fn not_logged_in(reason: &str) -> Reply {
    Reply::new(reply::code::USER_NOT_LOGGED_IN, reason)
}
