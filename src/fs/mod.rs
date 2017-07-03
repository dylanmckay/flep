//! The code for the generic `FileSystem` trait.
//!
//! Also contains both physical and in-memory implementations
//! of a file system.

pub use self::physical::Physical;
pub use self::memory::Memory;

mod physical;
mod memory;

use Error;
use std::path::Path;

/// A filesystem mountable as FTP.
pub trait FileSystem
{
    /// List all files/directories at a specific path.
    fn list(&self, path: &Path) -> Result<Vec<String>, Error>;

    /// Make a new directory.
    fn create_dir(&mut self, path: &Path) -> Result<(), Error>;

    /// Write data into a file.
    fn write_file(&mut self, path: &Path, data: Vec<u8>) -> Result<(), Error>;

    /// Read data from a file.
    fn read_file(&self, path: &Path) -> Result<Vec<u8>, Error>;
}

