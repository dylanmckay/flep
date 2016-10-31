use std::io;

#[derive(Debug)]
pub enum Error
{
    Io(io::Error),
}

impl From<io::Error> for Error
{
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
