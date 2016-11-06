use {Error, protocol};
use server::client::Action;

/// Handle the 'QUIT' command.
pub fn handle() -> Result<Action, Error> {
    Ok(Action::Reply(protocol::Reply::new(
        protocol::reply::code::SERVICE_CLOSING_CONTROL_CONNECTION,
        "goodbye")))
}
