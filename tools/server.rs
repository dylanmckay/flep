extern crate flep;

pub struct Server
{
    file_system: flep::server::fs::Physical,
}

impl flep::server::FileTransferProtocol for Server
{
    fn welcome_message(&self) -> String { "Hello there!".to_string() }

    fn file_system(&self) -> &flep::server::FileSystem {
        &self.file_system
    }

    fn file_system_mut(&mut self) -> &mut flep::server::FileSystem {
        &mut self.file_system
    }
}

fn main() {
    flep::server::run(Server {
        file_system: flep::server::fs::Physical::new("./"),
    });
}
