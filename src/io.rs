use Error;
use mio::*;

pub struct Io
{
    pub poll: Poll,
    token_accumulator: usize,
}

impl Io
{
    pub fn new() -> Result<Self, Error> {
        Ok(Io {
            poll: Poll::new()?,
            token_accumulator: 100,
        })
    }

    pub fn allocate_token(&mut self) -> Token {
        self.token_accumulator += 1;
        Token(self.token_accumulator)
    }
}
