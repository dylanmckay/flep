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
    /// FIXME: Maybe rename this to `create_dir` to be consistent with Rust.
    fn mkdir(&mut self, parent: &Path, name: String) -> Result<(), Error>;

    /// Write data into a file.
    /// FIXME: Come up with a better name for this and `read`.
    fn write(&mut self, path: &Path, data: Vec<u8>) -> Result<(), Error>;

    /// Read data from a file.
    fn read(&self, path: &Path) -> Result<Vec<u8>, Error>;
}

