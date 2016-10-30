use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct Credentials
{
    pub username: String,
    pub password: Option<String>,
}

// We have a custom Debug implementation so we don't print the password.
impl fmt::Debug for Credentials
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Credentials {{ username: {:?} }}", self.username)
    }
}
