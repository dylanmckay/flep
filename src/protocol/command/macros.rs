/// Defines an new raw FTP command.
macro_rules! define_command {
    ($name:ident { $( $arg_name:ident : $arg_ty:ty),* }) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name {
            $( pub $arg_name : $arg_ty ),*
        }

        impl $crate::Command for $name {
            #[allow(unused_variables)]
            fn write_payload(&self, write: &mut ::std::io::Write)
                -> Result<(), $crate::Error> {
                #[allow(unused_imports)]
                use $crate::Argument;

                $( self.$arg_name.write(write)?; )*
                Ok(())
            }

            #[allow(unused_variables)]
            fn read_payload(read: &mut ::std::io::BufRead)
                -> Result<Self, $crate::Error> {
                Ok($name {
                    $( $arg_name : <$arg_ty as $crate::Argument>::read_with_space(read)?, )*
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
