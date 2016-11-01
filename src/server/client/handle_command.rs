use {Credentials, DataTransfer, DataTransferMode, Error, FileType, Io};
use server::{Session, session};
use {server, protocol};

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
                    Ok(protocol::Reply::new(protocol::reply::code::USER_LOGGED_IN, "user logged in"))
                } else {
                    Ok(protocol::Reply::new(protocol::reply::code::USER_NOT_LOGGED_IN, "invalid credentials"))
                }
            } else {
                Err(protocol::Error::Client(protocol::ClientError::InvalidCommandSequence {
                    message: "the client must send password immediately after the username is sent".to_owned(),
                }).into())
            }
        },
        PWD(..) => {
            let session = client.session.expect_ready()?;

            Ok(protocol::Reply::new(protocol::reply::code::PATHNAME_CREATED,
                                 session.working_dir.clone().into_os_string().into_string().unwrap()))
        },
        CWD(ref cwd) => {
            let mut session = client.session.expect_ready_mut()?;

            session.working_dir = cwd.path.clone().into();
            Ok(protocol::Reply::new(protocol::reply::code::REQUESTED_FILE_ACTION_COMPLETED, "cwd changes"))
        },
        CDUP(..) => {
            let mut session = client.session.expect_ready_mut()?;

            session.working_dir = session.working_dir.parent().unwrap().to_owned();
            Ok(protocol::Reply::new(protocol::reply::code::REQUESTED_FILE_ACTION_COMPLETED, "cwd changes"))
        },
        LIST(..) => {
            client.initiate_transfer(server::Transfer {
                file_type: FileType::ascii(),
                data: "-rw-r--r-- 1 owner group           213 Aug 26 16:31 README\r\n".as_bytes().to_owned(),
            }).unwrap();

            if let DataTransfer::Connected { .. } = client.connection.dtp {
                Ok(protocol::Reply::new(125, "transfer starting"))
            } else {
                Ok(protocol::Reply::new(150, "about to open data connection"))
            }
        },
        // Client requesting information about the server system.
        SYST(..) => {
            Ok(protocol::Reply::new(protocol::reply::code::SYSTEM_NAME_TYPE, protocol::rfc1700::system::UNIX))
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

