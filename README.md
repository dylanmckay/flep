# flep

[![Crates.io](https://img.shields.io/crates/v/flep.svg)](https://crates.io/crates/flep)
[![Build Status](https://travis-ci.org/dylanmckay/flep.svg?branch=master)](https://travis-ci.org/dylanmckay/flep)
[![license](https://img.shields.io/github/license/dylanmckay/flep.svg)]()

An FTP server library.

[Documentation](https://docs.rs/flep)

## Example

Run `ftp 127.0.0.1 2222` to connect to the server.

```rust
pub struct Server
{
    file_system: flep::fs::Memory,
}

impl flep::server::Server for Server
{
    fn welcome_message(&self) -> String { "Hello there!".to_string() }

    fn file_system(&self) -> &flep::fs::FileSystem {
        &self.file_system
    }

    fn file_system_mut(&mut self) -> &mut flep::fs::FileSystem {
        &mut self.file_system
    }
}

fn main() {
    flep::util::log::initialize_default().expect("could not setup logging");

    // Set up an in-memory file system.
    let mut file_system = flep::fs::Memory::new();
    file_system.write_file(&Path::new("README.txt"),
                           "hello there\nit is me".as_bytes().to_owned()).unwrap();

    // Start on port 2222
    let mut server = Server { file_system: file_system };
    flep::server::run(&mut server, "127.0.0.1:2222")
        .expect("error whilst running server");
}
```

