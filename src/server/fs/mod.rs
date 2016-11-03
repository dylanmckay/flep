pub use self::physical::Physical;
pub use self::memory::Memory;

pub mod physical;
pub mod memory;

use Error;
use std::path::Path;

/// A filesystem mountable as FTP.
pub trait FileSystem
{
    fn list(&self, path: &Path) -> Result<Vec<String>, Error>;

    fn mkdir(&mut self, parent: &Path, name: String) -> Result<(), Error>;

    fn write(&mut self, path: &Path, data: Vec<u8>) -> Result<(), Error>;
}
