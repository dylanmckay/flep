define_command!(USER {
    username: String,
});

define_command!(PASS {
    password: String,
});

define_command!(CWD {
    path: String,
});

define_command!(ACCT {
    info: String,
});

define_command!(APPE {
    remote_filename: String,
});

define_command!(DELE {
    remote_filename: String,
});

define_command!(MDTM {
    remote_filename: String,
});

define_command!(MKD {
    remote_filename: String,
});

define_command!(RETR {
    remote_filename: String,
});

define_command!(RMD {
    remote_filename: String,
});

define_command!(RNFR {
    from_filename: String,
});

define_command!(RNTO {
    from_filename: String,
});

define_command!(SITE {
    command: String,
});

define_command!(SIZE {
    remote_filename: String,
});

define_command!(STOR {
    remote_filename: String,
});
