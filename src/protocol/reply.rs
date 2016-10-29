use std::io::prelude::*;
use std::{io, fmt};

/// A reply from the FTP server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reply
{
    /// The 3-digit reply code.
    pub code: u16,
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
    pub fn new<S>(code: u16, text: S) -> Self
        where S: Into<String> {
        let text: String = text.into();

        Reply {
            code: code,
            text: text.into(),
        }
    }

    pub fn write(&self, write: &mut Write) -> Result<(), io::Error> {
        match self.text {
            Text::SingleLine(ref line) => {
                write!(write, "{} {}\r\n", self.code, line)
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
