use server;

use std::collections::HashMap;
use uuid::Uuid;

/// An FTP server.
pub struct Server
{
    pub clients: HashMap<Uuid, server::Client>,
}

impl Server
{
    pub fn new() -> Self {
        Server { clients: HashMap::new() }
    }
}
