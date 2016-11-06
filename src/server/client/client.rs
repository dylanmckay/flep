use {Connection, server};

pub struct Client
{
    pub state: server::ClientState,
    pub connection: Connection,
}

