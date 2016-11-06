use {Credentials, Error};
use server::client::{Session, session};
use {server, protocol};

/// Handle the 'PASS' command.
pub fn handle(pass: &protocol::PASS,
              client: &mut server::Client,
              ftp: &mut server::FileTransferProtocol)
    -> Result<protocol::Reply, Error> {
    let session = client.session.expect_login()?.clone();

    if let session::Login::WaitingForPassword { username } = session {
        let credentials = Credentials { username: username.to_owned(), password: Some(pass.password.to_owned()) };

        if ftp.authenticate_user(&credentials) {
            client.session = Session::Ready(session::Ready::new(credentials));
            Ok(protocol::reply::pass::logged_in())
        } else {
            Ok(protocol::reply::pass::not_logged_in("invalid credentials"))
        }
    } else {
        Err(protocol::Error::Client(protocol::ClientError::InvalidCommandSequence {
            message: "the client must send password immediately after the username is sent".to_owned(),
        }).into())
    }
}
