#[derive(Clone, PartialEq, Eq)]
pub struct Credentials
{
    pub username: String,
    pub password: Option<String>,
}
