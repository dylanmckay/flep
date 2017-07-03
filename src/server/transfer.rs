//! The `Transfer` type.

use FileType;

/// A data transfer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transfer
{
    pub file_type: FileType,
    pub data: Vec<u8>,
}
