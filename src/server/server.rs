use Connection;
use server;

use std::collections::HashMap;
use uuid::Uuid;

/// An FTP server.
pub struct Server
{
    pub clients: HashMap<Uuid, ClientData>,
}

pub struct ClientData
{
    pub client: server::Client,
    pub connection: Connection,
}

impl Server
{
    pub fn new() -> Self {
        Server { clients: HashMap::new() }
    }
}
