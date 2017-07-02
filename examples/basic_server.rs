extern crate flep;

use flep::server::FileSystem;
use std::path::Path;

pub struct Server
{
    file_system: flep::server::fs::Memory,
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
    let mut file_system = flep::server::fs::Memory::new();
    file_system.write(&Path::new("README.txt"),
                      "hello there\nit is me".as_bytes().to_owned()).unwrap();

    let mut server = Server { file_system: file_system };
    flep::server::run(&mut server, "127.0.0.1:2222");
}
