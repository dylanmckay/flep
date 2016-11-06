mod user;
mod pass;
mod pwd;
mod cwd;
mod cdup;
mod list;
mod syst;
mod feat;
mod ty;
mod passive;
mod active;
mod quit;
mod retr;

use {Error, Io};
use {server, protocol};

/// Handles a command sent to a server from a client.
pub fn command(client: &mut server::Client,
               command: &protocol::CommandKind,
               ftp: &mut server::FileTransferProtocol,
               io: &mut Io) -> Result<protocol::Reply, Error> {
    use protocol::CommandKind::*;

    println!("received {:?}", command);

    match *command {
        // User attempting to log in.
        USER(ref user) => self::user::handle(user, client, ftp),
        PASS(ref pass) => self::pass::handle(pass, client, ftp),
        PWD(..) => self::pwd::handle(client),
        CWD(ref cwd) => self::cwd::handle(cwd, client),
        CDUP(..) => self::cdup::handle(client),
        LIST(ref list) => self::list::handle(list, client, ftp),
        // Client requesting information about the server system.
        SYST(..) => self::syst::handle(),
        FEAT(..) => self::feat::handle(),
        TYPE(ref ty) => self::ty::handle(ty, client),
        PASV(..) => self::passive::handle_pasv(client, io),
        EPSV(..) => self::passive::handle_epsv(client, io),
        PORT(ref port) => self::active::handle_port(port, client),
        QUIT(..) => self::quit::handle(),
        RETR(ref retr) => self::retr::handle(retr, client, ftp),
        ref command => {
            Err(Error::Protocol(protocol::ClientError::UnimplementedCommand {
                name: command.command_name().to_string(),
            }.into()))
        },
    }
}
