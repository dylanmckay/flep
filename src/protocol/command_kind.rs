use super::*;

use std::io::prelude::*;
use std::io;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandKind
{
    /// Abort an active file transfer.
    ABOR(ABOR),
    /// Account information.
    ACCT(ACCT),
    /// Authentication/Security Data.
    ADAT(ADAT),
    /// Allocate sufficient disk space to receive a file.
    ALLO(ALLO),
    /// Append.
    APPE(APPE),
    /// Authentication/Security Mechanism.
    AUTH(AUTH),
    /// Clear Command Channel.
    CCC(CCC),
    /// Change directory up one level.
    CDUP(CDUP),
    /// Confidentiality Protection Command.
    CONF(CONF),
    /// Change working directory.
    CWD(CWD),
    /// Delete file.
    DELE(DELE),
    /// Privacy Protected Channel.
    ENC(ENC),
    /// Specifies an extended address and port to which the server should connect.
    EPRT(EPRT),
    /// Enter extended passive mode.
    EPSV(EPSV),
    /// Get the feature list implemented by the server.
    FEAT(FEAT),
    /// Returns usage documentation on a command if specified, else a
    /// general help document is returned.
    HELP(HELP),
    /// Identify desired virtual host on server, by name.
    HOST(HOST),
    /// Language Negotiation.
    LANG(LANG),
    /// Returns information of a file or directory if specified, else
    /// information of the current working directory is returned.
    LIST(LIST),
    /// Specifies a long address and port to which the server should connect.
    LPRT(LPRT),
    /// Enter long passive mode.
    LPSV(LPSV),
    /// Return the last-modified time of a specified file.
    MDTM(MDTM),
    /// Integrity Protected Command.
    MIC(MIC),
    /// Make directory.
    MKD(MKD),
    /// Lists the contents of a directory if a directory is named.
    MLSD(MLSD),
    /// Provides data about exactly the object named on its command line,
    /// and no others.
    MLST(MLST),
    /// Sets the transfer mode.
    MODE(MODE),
    /// Returns a list of file names in a specified directory.
    NLST(NLST),
    /// A no-operation.
    NOOP(NOOP),
    /// Select options for a feature.
    OPTS(OPTS),
    /// Authentication password.
    PASS(PASS),
    /// Enter passive mode.
    PASV(PASV),
    /// Protection Buffer Size.
    PBSZ(PBSZ),
    /// Specifies an address and port to which the server should connect.
    PORT(PORT),
    /// Data Channel Protection Level.
    PROT(PROT),
    /// Gets the name of the current working directory on the remote host.
    PWD(PWD),
    /// Terminates the command connection.
    QUIT(QUIT),
    /// Reinitializes the command connection.
    REIN(REIN),
    /// Restart transfer from the specified point.
    REST(REST),
    /// Retrieve a copy of the file.
    RETR(RETR),
    /// Remove a directory.
    RMD(RMD),
    /// Rename from.
    RNFR(RNFR),
    /// Rename to.
    RNTO(RNTO),
    /// Sends site specific commands to remote server.
    SITE(SITE),
    /// Return the size of a file.
    SIZE(SIZE),
    /// Mount file structure.
    SMNT(SMNT),
    /// Returns the current status.
    STAT(STAT),
    /// Accept the data and to store the data as a file at the server site.
    STOR(STOR),
    /// Store file uniquely.
    STOU(STOU),
    /// Set file transfer structure.
    STRU(STRU),
    /// Returns a word identifying the system.
    SYST(SYST),
    /// Sets the transfer mode (ASCII/binary).
    TYPE(TYPE),
    /// Sets the name for the user.
    USER(USER),
    /// Change to the parent of the current working directory.
    XCUP(XCUP),
    /// Make a directory.
    XMKD(XMKD),
    /// Print the current working directory.
    XPWD(XPWD),
    XRCP(XRCP),
    /// Remove the directory.
    XRMD(XRMD),
    XRSQ(XRSQ),
    /// Send, mail if cannot.
    XSEM(XSEM),
    /// Send to terminal.
    XSEN(XSEN),
}

impl CommandKind
{
    /// Reads a command from a buffer.
    pub fn read(read: &mut Read) -> Result<Self, Error> {
        let line_bytes: Result<Vec<u8>, _> = read.bytes().take_while(|b| b.as_ref().map(|&b| (b as char) != '\n').unwrap_or(true)).collect();
        let mut line_bytes = line_bytes?;

        // Every new line should use '\r\n', and we trimmed the '\n' above.
        assert_eq!(line_bytes.last(), Some(&('\r' as u8)));
        line_bytes.pop();

        let line_string = String::from_utf8(line_bytes).unwrap();

        // Split the line up.
        let (command_name, payload) = if line_string.contains(' ') {
            let (command_name, payload) = line_string.split_at(line_string.chars().position(|c| c == ' ').expect("no space in line") + 1);

            // We don't want to look at the space character.
            (&command_name[0..command_name.len()-1], payload)
        } else {
            // If the line has no space, it has no payload.
            (line_string.as_str(), "")
        };

        let mut payload_reader = io::BufReader::new(io::Cursor::new(payload));

        macro_rules! read_commands {
            ( $cmd_name:ident => $( $name:ident ),+ ) => {
                match command_name {
                    $( stringify!($name) => Ok(CommandKind::$name($name::read_payload(&mut payload_reader)?)), )+
                    _ => Err(Error::InvalidCommand { name: command_name.to_owned() }),
                }
            }
        }

        read_commands!(command_name =>
            ABOR, ACCT, ADAT, ALLO, APPE, AUTH, CCC, CDUP, CONF, CWD, DELE, ENC,
            EPRT, EPSV, FEAT, HELP, HOST, LANG, LIST, LPRT, LPSV, MDTM, MIC, MKD,
            MLSD, MLST, MODE, NLST, NOOP, OPTS, PASS, PASV, PBSZ, PORT, PROT, PWD,
            QUIT, REIN, REST, RETR, RMD, RNFR, RNTO, SITE, SIZE, SMNT, STAT, STOR,
            STOU, STRU, SYST, TYPE, USER, XCUP, XMKD, XPWD, XRCP, XRMD, XRSQ, XSEM,
            XSEN
        )
    }
}
