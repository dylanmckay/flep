use server::Client;

use std::collections::HashMap;
use uuid::Uuid;

/// An FTP server.
pub struct Server
{
    pub clients: HashMap<Uuid, Client>,
}

impl Server
{
    pub fn new() -> Self {
        Server { clients: HashMap::new() }
    }
}
