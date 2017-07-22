use {Error, protocol};
use server::client::Action;

/// Handle the 'SYST' command.
pub fn handle() -> Result<Action, Error> {
    Ok(Action::Reply(protocol::reply::syst::success(protocol::rfc1700::system::UNIX.to_owned())))
}
