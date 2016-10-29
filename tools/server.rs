extern crate flep;

pub struct Server;

impl flep::server::FileTransferProtocol for Server
{
    fn welcome_message(&self) -> String { "Hello there!".to_string() }
}

impl flep::server::FileSystem for Server { }

fn main() {
    flep::server::run(Server);
}
