//! Raw FTP protocol definitions.
//!
//! * [RFC 959](https://www.w3.org/Protocols/rfc959)
//! * http://www.nsftools.com/tips/RawFTP.htm

extern crate itertools;
extern crate byteorder;

pub use self::command_kind::{CommandKind, Command};
pub use self::argument::Argument;
pub use self::reply::Reply;
pub use self::command::*;

/// Defines an new raw FTP command.
macro_rules! define_command {
    ($name:ident { $( $arg_name:ident : $arg_ty:ty),* }) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name {
            $( pub $arg_name : $arg_ty ),*
        }

        impl $crate::Command for $name {
            fn write_payload(&self, write: &mut ::std::io::Write)
                -> Result<(), ::std::io::Error> {
                $( self.$arg_name.write(write)?; )*
                Ok(())
            }

            fn read_payload(read: &mut ::std::io::BufRead)
                -> Result<Self, ::std::io::Error> {
                Ok($name {
                    $( $arg_name : <$arg_ty as $crate::Argument>::read(read)?, )*
                })
            }

            fn command_name(&self) -> &'static str { stringify!($name) }
        }
    };

    // Allow trailing commas.
    ($name:ident { $( $arg_name:ident : $arg_ty:ty),* , }) => {
        define_command!($name { $( $arg_name : $arg_ty ),* });
    };
}

pub mod command_kind;
pub mod argument;
pub mod reply;
pub mod command;

