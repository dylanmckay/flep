use protocol;

error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }

    links {
        Protocol(protocol::Error, protocol::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
    }
}

