pub use self::code::Code;

use std::io::prelude::*;
use std::{io, fmt};

/// A reply from the FTP server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reply
{
    /// The 3-digit reply code.
    pub code: Code,
    /// The response text.
    pub text: Text,
}

/// Reply text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Text
{
    /// The reply is only a single line of text.
    SingleLine(String),
    /// The reply is multiple lines of text.
    MultiLine(Vec<String>),
}

impl Reply
{
    pub fn new<C,S>(code: C, text: S) -> Self
        where C: Into<Code>, S: Into<String> {
        let text: String = text.into();

        Reply {
            code: code.into(),
            text: text.into(),
        }
    }

    pub fn write(&self, write: &mut Write) -> Result<(), io::Error> {
        match self.text {
            Text::SingleLine(ref line) => {
                write!(write, "{} {}\r\n", self.code.0, line)
            },
            Text::MultiLine(..) => unimplemented!(),
        }
    }
}

impl From<String> for Text
{
    fn from(s: String) -> Text {
        let lines: Vec<_> = s.lines().collect();
        assert_eq!(lines.is_empty(), false);

        if lines.len() == 1 {
            Text::SingleLine(lines[0].to_owned())
        } else {
            Text::MultiLine(lines.into_iter().map(|l| l.to_owned()).collect())
        }
    }
}

impl fmt::Display for Text
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Text::SingleLine(ref line) => write!(fmt, "{}", line),
            Text::MultiLine(ref lines) => {
                for line in lines { write!(fmt, "{}\n", line)?; }
                Ok(())
            },
        }
    }
}

/// Reply code definitions.
pub mod code
{
    /// A reply code.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Code(pub u16);

    pub const OK: Code = Code(200);
    pub const INVALID_COMMAND: Code = Code(500);
    pub const SYNTAX_ERROR: Code = Code(501);
    pub const COMMAND_NOT_IMPLEMENTED_SUPERFLOUS: Code = Code(202);
    pub const COMMAND_NOT_IMPLEMENTED: Code = Code(502);
    pub const BAD_COMMAND_SEQUENCE: Code = Code(503);
    pub const COMMAND_NOT_IMPLEMENTED_FOR_PARAMETE: Code = Code(504);
    pub const RESTART_MARKER_REPLY: Code = Code(110);
    pub const STATUS_OR_HELP_REPLY: Code = Code(211);
    pub const DIRECTORY_STATUS: Code = Code(212);
    pub const FILE_STATUS: Code = Code(213);
    pub const HELP_MESSAGE: Code = Code(214);
    pub const SYSTEM_NAME_TYPE: Code = Code(215);
    pub const SERVICE_READY_ETA: Code = Code(120);
    pub const SERVICE_READY_FOR_NEW_USER: Code = Code(220);
    pub const SERVICE_CLOSING_CONTROL_CONNECTION: Code = Code(221);
    pub const SERVICE_UNAVAILABLE_CLOSING_CONTROL_CONNECTION: Code = Code(421);
    pub const DATA_CONNECTION_ALREADY_OPEN_TRANSFER_STARTING: Code = Code(125);
    pub const DATA_CONNECTION_OPEN_NO_TRANSFER_IN_PROGRESS: Code = Code(225);
    pub const CANT_OPEN_DATA_CONNECTION: Code = Code(425);
    pub const CLOSING_DATA_CONNECTION: Code = Code(226);
    pub const CONNECTION_CLOSED_TRANSFER_ABORTED: Code = Code(426);
    pub const ENTERING_PASSIVE_MODE: Code = Code(227);
    pub const USER_LOGGED_IN: Code = Code(230);
    pub const USER_NOT_LOGGED_IN: Code = Code(530);
    pub const USER_NAME_OKAY_NEED_PASSWORD: Code = Code(331);
    pub const NEED_ACCOUNT_FOR_LOGIN: Code = Code(332);
    pub const NEED_ACCOUNT_FOR_STORING_FILES: Code = Code(532);
    pub const FILE_STATUS_OKAY: Code = Code(150);
    pub const REQUESTED_FILE_ACTION_COMPLETED: Code = Code(250);
    pub const PATHNAME_CREATED: Code = Code(257);
    pub const REQUESTED_FILE_ACTION_PENDING_FURTHER_INFORMATION: Code = Code(350);
    pub const REQUESTED_FILE_ACTION_NOT_TAKEN: Code = Code(450);
    pub const REQUESTED_ACTION_NOT_TAKEN: Code = Code(550);
    pub const REQUESTED_ACTION_ABORTED_LOCAL_ERROR_IN_PROCESSING: Code = Code(451);
    pub const REQUESTED_ACTION_ABORTED_PAGE_TYPE_UNKNOWN: Code = Code(551);
    pub const REQUESTED_ACTION_NOT_TAKEN_INSUFFICIENT_STORAGE: Code = Code(452);
    pub const REQUESTED_FILE_ACTION_ABORTED_EXCEEDED_ALLOCATION: Code = Code(552);
    pub const INVALID_FILE_NAME: Code = Code(553);

    impl Into<Code> for u16 {
        fn into(self) -> Code { Code(self) }
    }
}
