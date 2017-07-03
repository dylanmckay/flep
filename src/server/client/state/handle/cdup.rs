use {Error, protocol};
use server::client::{Action, ClientState};

/// Handle the 'CDUP' command.
pub fn handle(client: &mut ClientState) -> Result<Action, Error> {
    let mut session = client.session.expect_ready_mut()?;

    match session.working_dir.parent().map(ToOwned::to_owned) {
        Some(parent) => {
            session.working_dir = parent;
            Ok(Action::Reply(protocol::reply::cdup::success()))
        },
        None => Ok(Action::Reply(protocol::reply::cdup::no_parent()))
    }
}
