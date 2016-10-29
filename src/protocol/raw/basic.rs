use raw::Command;
use std::io::prelude::*;
use std::io;

/// Defines a packet which takes no arguments.
macro_rules! define_basic_command {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name;

        impl Command for $name
        {
            fn write_payload(&self, _: &mut Write) -> Result<(), io::Error> { Ok(()) }
            fn read_payload(_: &mut BufRead) -> Result<Self, io::Error> { Ok($name) }

            fn command_name(&self) -> &'static str { stringify!($name) }
        }

        #[cfg(test)]
        mod test
        {
            #[allow(non_snake_case)]
            mod $name {
                use raw::*;
                use std::io;

                #[test]
                fn correctly_writes_basic_packets() {
                    let packet = $name;
                    let raw_bytes = packet.bytes();
                    let text = String::from_utf8(raw_bytes).unwrap();

                    assert_eq!(text, stringify!($name));
                }

                #[test]
                fn correctly_reads_basic_packets() {
                    let raw_bytes = stringify!($name).as_bytes();
                    let command = CommandKind::read(&mut io::Cursor::new(raw_bytes.to_vec())).unwrap();

                    assert_eq!(command, CommandKind::$name($name));
                }
            }
        }
    }
}

define_basic_command!(PASV);
