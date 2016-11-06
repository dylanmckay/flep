use Connection;
use server;

use std::collections::HashMap;
use uuid::Uuid;

/// An FTP server.
pub struct Server
{
    pub clients: HashMap<Uuid, Client>,
}

pub struct Client
{
    pub state: server::ClientState,
    pub connection: Connection,
}

impl Server
{
    pub fn new() -> Self {
        Server { clients: HashMap::new() }
    }
}
