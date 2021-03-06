error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }

    foreign_links {
        Io(::std::io::Error);
        InvalidUtf8(::std::string::FromUtf8Error);
    }

    errors {
        NotLoggedIn {
            description("client not logged in")
            display("client not logged in")
        }

        InvalidCommand(name: String) {
            description("received invalid command")
            display("received invalid command: '{}'", name)
        }

        InvalidArgument(message: String) {
            description("received invalid argument")
            display("received invalid argument: {}", message)
        }

        InvalidCommandSequence(message: String) {
            description("received invalid command sequence")
            display("received invalid command sequence: {}", message)
        }

        UnimplementedCommand(name: String) {
            description("received command that is not implemented yet")
            display("received command that is not implemented yet: '{}'", name)
        }
    }
}

