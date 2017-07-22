/// Defines a module containing methods which build replies
/// for different scenarios.
macro_rules! define_replies {
    ($cmd_name:ident {
        $( $reply_name:ident ( $( $arg_name:ident : $arg_ty:ty ),*  ) => $code_name:ident @ $message:expr ),+
    }) => {
        pub mod $cmd_name {
            use {Reply, reply};
            #[allow(unused_imports)]
            use std::path::Path;

            $(
                pub fn $reply_name( $( $arg_name : $arg_ty )* ) -> Reply {
                    Reply::new(reply::code::$code_name, $message)
                }
            )+
        }
    };

}

// NOTE: Keep commands in ALPHABETICAL ORDER.

define_replies!(cdup {
    success() => REQUESTED_FILE_ACTION_COMPLETED @ "changed to parent directory",
    no_parent() => REQUESTED_ACTION_NOT_TAKEN @ "there is no parent directory"
});

define_replies!(cwd {
    success() => REQUESTED_FILE_ACTION_COMPLETED @ "changed working directory"
});

define_replies!(epsv {
    success(port: u16) => ENTERING_PASSIVE_MODE_EXTENDED
        @ format!("passive mode enabled (|||{}|)", port)
});

define_replies!(mkd {
    success() => PATHNAME_CREATED @ "created directory"
});

define_replies!(pass {
    logged_in() => USER_LOGGED_IN @ "user logged in",
    not_logged_in(reason: &str) => USER_NOT_LOGGED_IN @ reason
});

define_replies!(pasv {
    success(port: u16) => ENTERING_PASSIVE_MODE
        @ format!("passive mode enabled (127,0,0,1,{})", port)
});

define_replies!(pwd {
    // It's pretty weird that 'PWD' returns 'PATHNAME_CREATED' on
    // success. Here's what RFC 959 has to say:
    //
    // "Essentially because the PWD command returns the same type of
    // information as the successful MKD command, the successful PWD
    // command uses the 257 reply code as well."
    success(working_dir: &Path) => PATHNAME_CREATED
        @ format!("\"{}\"", working_dir.display())
});

define_replies!(syst {
    // * `os` is the operating system. It should be one of the
    // assigned constants from RFC 943.
    success(os: String) => SYSTEM_NAME_TYPE @ os
});

define_replies!(user {
    logged_in() => USER_LOGGED_IN @ "user logged in",
    need_password() => USER_NAME_OKAY_NEED_PASSWORD @ "need password",
    not_logged_in(reason: &str) => USER_NOT_LOGGED_IN @ reason
});

