use {Error, ClientError};

use std::io::prelude::*;
use std::ascii::AsciiExt;
use std::io;

/// An argument to a command.
pub trait Argument : Sized
{
    /// Read an argument with a leading space character.
    ///
    /// Most argument types don't care about leading spaces. These
    /// types can instead override `read`.
    fn read_with_space(read: &mut BufRead) -> Result<Self, Error> {
        let mut buf: [u8; 1] = [0];

        // Check if we can read a byte
        assert_eq!(read.read(&mut buf)?, 1, "unexpected EOF while checking for space");
        assert_eq!(buf[0] as char, ' ', "expected space preceding argument");

        // Now that we've trimmed the space, delegate downwards.
        Self::read(read)
    }

    fn read(read: &mut BufRead) -> Result<Self, Error>;
    fn write(&self, write: &mut Write) -> Result<(), Error>;

    fn parse_text(text: &str) -> Self {
        let mut buffer = io::Cursor::new(text);
        Self::read_with_space(&mut buffer).unwrap()
    }

    fn bytes(&self) -> Vec<u8> {
        let mut buffer = io::Cursor::new(Vec::new());
        self.write(&mut buffer).unwrap();
        buffer.into_inner()
    }

    fn to_string(&self) -> String {
        String::from_utf8(self.bytes()).unwrap()
    }
}

impl Argument for String
{
    fn read(read: &mut BufRead) -> Result<Self, Error> {
        let bytes: Result<Vec<u8>, _> = read.bytes().collect();
        let bytes = bytes?;

        match String::from_utf8(bytes) {
            Ok(s) => Ok(s),
            Err(..) => Err(Error::Client(ClientError::InvalidArgument {
                message: "argument is not valid UTF-8".to_owned() ,
            })),
        }
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        for c in self.chars() { assert!(c.is_ascii(), "only ASCII is supported in FTP"); }
        write!(write, "{}", self)?;
        Ok(())
    }
}

// Optional arguments are always at the end of the command.
//
// We know that if there is a trailing space after the previously
// parsed argument, the optional argument must be present.
impl<T: Argument> Argument for Option<T>
{
    fn read_with_space(read: &mut BufRead) -> Result<Self, Error> {
        let mut buf: [u8; 1] = [0];

        // Check if we can read a byte
        if read.read(&mut buf)? == 1 {
            let inner = T::read(read)?;
            Ok(Some(inner))
        } else {
            Ok(None)
        }
    }

    fn read(_read: &mut BufRead) -> Result<Self, Error> {
        // We override a higher level method - read_with_space
        unreachable!();
    }

    fn write(&self, write: &mut Write) -> Result<(), Error> {
        if let Some(ref thing) = *self {
            write!(write, " ")?;
            thing.write(write)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test
{
    pub use super::*;

    mod optional
    {
        use std::io;
        pub use super::*;

        fn parse<T: Argument>(text: &str) -> Option<T> {
            let mut buf = io::Cursor::new(text);
            Argument::read_with_space(&mut buf).unwrap()
        }

        #[test]
        fn correctly_reads_a_present_value() {
            let value: Option<String> = parse(" foo");
            assert_eq!(value, Some("foo".to_owned()));
        }

        #[test]
        fn correctly_reads_a_missing_value() {
            let value: Option<String> = parse("");
            assert_eq!(value, None);
        }

        #[test]
        fn correctly_writes_a_present_value() {
            assert_eq!(Some("foo".to_owned()).to_string(), " foo");
        }

        #[test]
        fn correctly_writes_an_empty_value() {
            let value: Option<String> = None;
            assert_eq!(value.to_string(), "");
        }
    }
}

