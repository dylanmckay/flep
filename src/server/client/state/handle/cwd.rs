use {Error, protocol};
use server::client::{ClientState, Action};

/// Handle the 'CWD' command.
pub fn handle(cwd: &protocol::CWD,
              client: &mut ClientState) -> Result<Action, Error> {
    let mut session = client.session.expect_ready_mut()?;

    session.working_dir = cwd.path.clone().into();
    Ok(Action::Reply(protocol::reply::cwd::success()))
}
