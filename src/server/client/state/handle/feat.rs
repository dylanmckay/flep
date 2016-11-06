use {Error, protocol};
use server::client::Action;

/// Handle the 'FEAT' command.
pub fn handle() -> Result<Action, Error> {
    Ok(Action::Reply(protocol::reply::feat::Features::default().into()))
}
