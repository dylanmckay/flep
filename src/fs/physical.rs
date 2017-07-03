use Error;
use super::FileSystem;

use std::path::{Path, PathBuf};
use std::fs;

/// A folder on the physical on-disk filesystem.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Physical
{
    /// The root directory.
    pub root: PathBuf,
}

impl Physical
{
    /// Creates a new physical filesystem.
    pub fn new<P>(root: P) -> Self
        where P: Into<PathBuf> {
        Physical {
            root: root.into(),
        }
    }
}

impl FileSystem for Physical
{
    fn list(&self, path: &Path) -> Result<Vec<String>, Error> {
        let full_path = self.root.join(path);

        let entries: Result<Vec<fs::DirEntry>, _> = fs::read_dir(&full_path)?.collect();
        let entries = entries?;

        let names: Vec<String>= entries.into_iter().map(|entry| {
            let base_name = entry.path().strip_prefix(&full_path).unwrap().to_owned();
            base_name.to_str().unwrap().to_owned()
        }).collect();

        Ok(names)
    }

    fn create_dir(&mut self, _path: &Path) -> Result<(), Error> {
        unimplemented!();
    }

    fn write_file(&mut self, _path: &Path, _data: Vec<u8>) -> Result<(), Error> {
        unimplemented!();
    }

    fn read_file(&self, _path: &Path) -> Result<Vec<u8>, Error> {
        unimplemented!();
    }
}
