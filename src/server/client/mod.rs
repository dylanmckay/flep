//! Data structures for representing a client from server's POV.

pub use self::client::Client;
pub use self::state::{Session, ClientState};
pub use self::action::Action;

pub mod client;
pub mod state;
pub mod action;

mod client_io;
