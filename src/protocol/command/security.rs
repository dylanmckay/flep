//! FTP security extensions from RFC 2228.
//!
//! * [RFC 2228](https://tools.ietf.org/html/rfc2228)

define_command!(AUTH {
    auth_type: String,
});

define_command!(ADAT {
    auth_data: String,
});

define_command!(PBSZ {
    protection_buffer_size: u32,
});

// FIXME: Turn protection_level into an enum.
// C - Clear
// S - Safe
// E - Confidential
// P - Private
define_command!(PROT {
    protection_level: String,
});

define_command!(CCC { });

define_command!(MIC {
    message: String,
});

define_command!(CONF {
    message: String,
});

define_command!(ENC {
    message: String,
});
