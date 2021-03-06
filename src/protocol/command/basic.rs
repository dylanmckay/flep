/// Defines a packet which takes no arguments.
macro_rules! define_basic_command {
    ($name:ident, $module_name:ident) => {
        pub use self::$module_name::$name;

        pub mod $module_name {
            use Command;
            use std::io::prelude::*;

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct $name;

            impl Command for $name
            {
                fn write_payload(&self, _: &mut Write) -> Result<(), $crate::Error> { Ok(()) }
                fn read_payload(_: &mut BufRead) -> Result<Self, $crate::Error> { Ok($name) }

                fn command_name(&self) -> &'static str { stringify!($name) }
            }

            #[cfg(test)]
            mod test
            {
                use super::*;
                use {Command, CommandKind};
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
                    let raw_text = format!("{}\r\n", stringify!($name));
                    let command = CommandKind::read(&mut io::Cursor::new(raw_text.as_bytes().to_vec())).unwrap();

                    assert_eq!(command, CommandKind::$name($name));
                }
            }
        }
    }
}

// Abort the current file transfer.
define_basic_command!(ABOR, abor);
// Change directory up one level.
define_basic_command!(CDUP, cdup);
// Get the feature list implemented by the server.
define_basic_command!(FEAT, feat);
// Extended passive mode.
define_basic_command!(EPSV, epsv);
// A no-operation.
define_basic_command!(NOOP, noop);
// Enable passive mode.
define_basic_command!(PASV, pasv);
// Gets the name of the current working directory on the remote host.
define_basic_command!(PWD, pwd);
// Terminates the command connection.
define_basic_command!(QUIT, quit);
// Reinitializes the command connectio.
define_basic_command!(REIN, rein);
// Begins transmission of a file to the remote site.
define_basic_command!(STOU, stou);
// Returns a word identifying the system.
define_basic_command!(SYST, syst);
