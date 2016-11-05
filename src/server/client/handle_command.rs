use {Credentials, DataTransfer, DataTransferMode, Error, FileType, Io};
use server::{Session, session};
use {server, protocol};

use std::path::Path;

/// Handles a command sent to a server from a client.
pub fn handle(client: &mut server::Client,
              command: protocol::CommandKind,
              ftp: &mut server::FileTransferProtocol,
              io: &mut Io) -> Result<protocol::Reply, Error> {
    use protocol::CommandKind::*;

    println!("received {:?}", command);

    match command {
        // User attempting to log in.
        USER(ref user) => {
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
        },
        PASS(ref pass) => {
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
        },
        PWD(..) => {
            let session = client.session.expect_ready()?;
            Ok(protocol::reply::pwd::success(&session.working_dir))
        },
        CWD(ref cwd) => {
            let mut session = client.session.expect_ready_mut()?;

            session.working_dir = cwd.path.clone().into();
            Ok(protocol::reply::cwd::success())
        },
        CDUP(..) => {
            let mut session = client.session.expect_ready_mut()?;

            session.working_dir = session.working_dir.parent().unwrap().to_owned();
            Ok(protocol::reply::cdup::success())
        },
        LIST(..) => {
            let working_dir = client.session.expect_ready()?.working_dir.clone();

            let entries = ftp.file_system().list(&working_dir)?;
            let mut data: String = entries.join("\r\n");
            data.extend("\r\n".chars());

            Ok(client.initiate_transfer(server::Transfer {
                file_type: FileType::ascii(),
                data: data.as_bytes().to_owned(),
            }))
        },
        // Client requesting information about the server system.
        SYST(..) => {
            Ok(protocol::reply::syst::successful(protocol::rfc1700::system::UNIX.to_owned()))
        },
        FEAT(..) => {
            Ok(protocol::reply::feat::Features::default().into())
        },
        TYPE(ref ty) => {
            let mut session = client.session.expect_ready_mut()?;

            session.transfer_type = ty.file_type;

            println!("file type set to {:?}", ty.file_type);
            Ok(protocol::Reply::new(protocol::reply::code::OK, "file type set"))
        },
        PASV(..) => {
            let port = listen_passive_dtp(client, io)?;
            Ok(protocol::reply::pasv::success(port))
        },
        EPSV(..) => {
            let port = listen_passive_dtp(client, io)?;
            Ok(protocol::reply::epsv::success(port))
        },
        PORT(ref port) => {
            let mut session = client.session.expect_ready_mut()?;

            println!("set port to {}", port.port);
            session.port = Some(port.port);
            Ok(protocol::Reply::new(protocol::reply::code::OK, "port"))
        },
        QUIT(..) => {
            Ok(protocol::Reply::new(protocol::reply::code::SERVICE_CLOSING_CONTROL_CONNECTION,
                                    "goodbye"))
        },
        RETR(ref retr) => {
            let data = ftp.file_system().read(&Path::new(&retr.remote_filename))?;

            Ok(client.initiate_transfer(server::Transfer {
                file_type: FileType::ascii(),
                data: data,
            }))
        },
        command => {
            Err(Error::Protocol(protocol::ClientError::UnimplementedCommand {
                name: command.command_name().to_string(),
            }.into()))
        },
    }
}

/// Attempts to open a data connection passively.
fn listen_passive_dtp(client: &mut server::Client, io: &mut Io)
    -> Result<u16, Error> {
    let mut session = client.session.expect_ready_mut()?;
    let port = 5166;

    session.data_transfer_mode = DataTransferMode::Passive { port: port };
    client.connection.dtp = DataTransfer::bind(port, io).unwrap();
    Ok(port)
}
