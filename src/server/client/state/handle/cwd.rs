use {Error, server, protocol};
use server::client::Action;

/// Handle the 'CWD' command.
pub fn handle(cwd: &protocol::CWD,
              client: &mut server::ClientState) -> Result<Action, Error> {
    let mut session = client.session.expect_ready_mut()?;

    session.working_dir = cwd.path.clone().into();
    Ok(Action::Reply(protocol::reply::cwd::success()))
}
