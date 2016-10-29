use Connection;

use uuid::Uuid;

/// An FTP client from the point-of-view of the FTP server.
pub struct Client
{
    pub uuid: Uuid,
    pub connection: Connection,
}
