use {Credentials, Error};
use server::Server;
use server::client::state::{Session, session};
use server::client::{ClientState, Action};
use protocol;

/// Handle the 'PASS' command.
pub fn handle(pass: &protocol::PASS,
              client: &mut ClientState,
              server: &mut Server)
    -> Result<Action, Error> {
    let session = client.session.expect_login()?.clone();

    if let session::Login::WaitingForPassword { username } = session {
        let credentials = Credentials { username: username.to_owned(), password: Some(pass.password.to_owned()) };

        if server.authenticate_user(&credentials) {
            client.session = Session::Ready(session::Ready::new(credentials));
            Ok(Action::Reply(protocol::reply::pass::logged_in()))
        } else {
            Ok(Action::Reply(protocol::reply::pass::not_logged_in("invalid credentials")))
        }
    } else {
        Err(protocol::Error::from_kind(protocol::ErrorKind::InvalidCommandSequence(
            "the client must send password immediately after the username is sent".to_owned()
        )).into())
    }
}
