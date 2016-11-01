pub use self::code::Code;

pub mod code;

pub mod feat;
pub mod epsv;
pub mod pasv;

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

    pub fn single_line<C,S>(code: C, text: S) -> Self
        where C: Into<Code>, S: Into<String> {
        Reply {
            code: code.into(),
            text: Text::SingleLine(text.into()),
        }
    }

    pub fn multi_line<C>(code: C, lines: Vec<String>) -> Self
        where C: Into<Code> {
        Reply {
            code: code.into(),
            text: Text::MultiLine(lines),
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

