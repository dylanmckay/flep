use {Credentials, Error};
use server::client::{Session, session};
use {server, protocol};

/// Handle the 'USER' command.
pub fn handle(user: &protocol::USER,
              client: &mut server::Client,
              ftp: &mut server::FileTransferProtocol)
    -> Result<protocol::Reply, Error> {
    let session = client.session.expect_login()?.clone();

    if let session::Login::WaitingForUsername = session {
        let credentials = Credentials { username: user.username.to_owned(), password: None };

        // The user may authenticate with no password
        if ftp.authenticate_user(&credentials) {
            client.session = Session::Ready(session::Ready::new(credentials));
            Ok(protocol::reply::user::logged_in())
        } else {
            // The user needs a password to get through.
            client.session = Session::Login(session::Login::WaitingForPassword {
                username: user.username.to_owned(),
            });

            Ok(protocol::reply::user::need_password())
        }
    } else {
        Err(protocol::Error::Client(protocol::ClientError::InvalidCommandSequence {
            message: "the client wait until we send the welcome message to log in".to_owned(),
        }).into())
    }
}
