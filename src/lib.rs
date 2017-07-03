pub extern crate flep_protocol as protocol;

extern crate mio;
extern crate uuid;
#[macro_use]
extern crate error_chain;

pub use self::credentials::Credentials;
pub use self::errors::*;

// Reexports.
pub use protocol::FileType;

pub mod server;
pub mod credentials;
pub mod errors;
pub mod io;
