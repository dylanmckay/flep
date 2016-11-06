use {Error, server, protocol};
use server::client::Action;

/// Handle the 'PWD' command.
pub fn handle(client: &mut server::ClientState) -> Result<Action, Error> {
    let session = client.session.expect_ready()?;
    Ok(Action::Reply(protocol::reply::pwd::success(&session.working_dir)))
}
