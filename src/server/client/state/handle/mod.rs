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
mod mkd;

use Error;
use server::client::Action;
use {server, protocol};

/// Handles a command sent to a server from a client.
pub fn command(client: &mut server::ClientState,
               command: &protocol::CommandKind,
               ftp: &mut server::FileTransferProtocol)
    -> Result<server::client::Action, Error> {
    use protocol::CommandKind::*;

    println!("received {:?}", command);

    match *command {
        // User attempting to log in.
        USER(ref user) => self::user::handle(user, client, ftp),
        PASS(ref pass) => self::pass::handle(pass, client, ftp),
        PWD(..) => self::pwd::handle(client),
        CWD(ref cwd) => self::cwd::handle(cwd, client),
        CDUP(..) => self::cdup::handle(client),
        MKD(ref mkd) => self::mkd::handle(mkd, client, ftp),
        LIST(ref list) => self::list::handle(list, client, ftp),
        // ClientState requesting information about the server system.
        SYST(..) => self::syst::handle(),
        FEAT(..) => self::feat::handle(),
        TYPE(ref ty) => self::ty::handle(ty, client),
        PASV(..) => self::passive::handle_pasv(client),
        EPSV(..) => self::passive::handle_epsv(client),
        PORT(ref port) => self::active::handle_port(port, client),
        QUIT(..) => self::quit::handle(),
        RETR(ref retr) => self::retr::handle(retr, client, ftp),
        EPRT(..) => self::unimplemented("EPRT"),
        ABOR(..) => self::unimplemented("ABOR"),
        ACCT(..) => self::unimplemented("ACCT"),
        ADAT(..) => self::unimplemented("ADAT"),
        ALLO(..) => self::unimplemented("ALLO"),
        APPE(..) => self::unimplemented("APPE"),
        AUTH(..) => self::unimplemented("AUTH"),
        CCC(..) => self::unimplemented("CCC"),
        CONF(..) => self::unimplemented("CONF"),
        DELE(..) => self::unimplemented("DELE"),
        ENC(..) => self::unimplemented("ENC"),
        HELP(..) => self::unimplemented("HELP"),
        HOST(..) => self::unimplemented("HOST"),
        LANG(..) => self::unimplemented("LANG"),
        LPRT(..) => self::unimplemented("LPRT"),
        LPSV(..) => self::unimplemented("LPSV"),
        MDTM(..) => self::unimplemented("MDTM"),
        MIC(..) => self::unimplemented("MIC"),
        MLSD(..) => self::unimplemented("MLSD"),
        MLST(..) => self::unimplemented("MLST"),
        MODE(..) => self::unimplemented("MODE"),
        NLST(..) => self::unimplemented("NLST"),
        NOOP(..) => self::unimplemented("NOOP"),
        OPTS(..) => self::unimplemented("OPTS"),
        PBSZ(..) => self::unimplemented("PBSZ"),
        PROT(..) => self::unimplemented("PROT"),
        REIN(..) => self::unimplemented("REIN"),
        REST(..) => self::unimplemented("REST"),
        RMD(..) => self::unimplemented("RMD"),
        RNFR(..) => self::unimplemented("RNFR"),
        RNTO(..) => self::unimplemented("RNTO"),
        SITE(..) => self::unimplemented("SITE"),
        SIZE(..) => self::unimplemented("SIZE"),
        SMNT(..) => self::unimplemented("SMNT"),
        STAT(..) => self::unimplemented("STAT"),
        STOR(..) => self::unimplemented("STOR"),
        STOU(..) => self::unimplemented("STOU"),
        STRU(..) => self::unimplemented("STRU"),
        XCUP(..) => self::unimplemented("XCUP"),
        XMKD(..) => self::unimplemented("XMKD"),
        XPWD(..) => self::unimplemented("XPWD"),
        XRCP(..) => self::unimplemented("XRCP"),
        XRMD(..) => self::unimplemented("XRMD"),
        XRSQ(..) => self::unimplemented("XRSQ"),
        XSEM(..) => self::unimplemented("XSEM"),
        XSEN(..) => self::unimplemented("XSEN"),
    }
}

/// Generate a reply for an unimplemented command.
fn unimplemented(command_name: &'static str) -> Result<Action, Error> {
    Err(protocol::Error::from_kind(protocol::ErrorKind::UnimplementedCommand(
        command_name.to_string(),
    )).into())
}
