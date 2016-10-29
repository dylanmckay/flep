//! Raw FTP command definitions.
//!
//! http://www.nsftools.com/tips/RawFTP.htm

pub use self::kind::{CommandKind, Command};
pub use self::argument::Argument;
pub use self::port::PORT;
pub use self::basic::{ABOR, CDUP, NOOP, PASV, PWD, QUIT, REIN, STOU, SYST};

pub mod kind;
pub mod argument;
pub mod port;
/// Commands which take no arguments.
pub mod basic;

/// Defines an new raw FTP command.
macro_rules! define_command {
    ($name:ident { $( $arg_name:ident : $arg_ty:ty),* }) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name {
            $( pub $arg_name : $arg_ty ),+
        }

        impl $crate::raw::Command for $name {
            fn write_payload(&self, write: &mut ::std::io::Write)
                -> Result<(), ::std::io::Error> {
                $( self.$arg_name.write(write)?; )+
                Ok(())
            }

            fn read_payload(read: &mut ::std::io::BufRead)
                -> Result<Self, ::std::io::Error> {
                Ok($name {
                    $( $arg_name : <$arg_ty as $crate::raw::Argument>::read(read)?, )+
                })
            }

            fn command_name(&self) -> &'static str { stringify!($name) }
        }
    }
}

define_command!(FOOBAR {
    value: String
});

