use Error;
use server::FileSystem;

use std::path::{Path, PathBuf};
use std::fs;

/// A folder on the physical filesystem.
pub struct Physical
{
    pub root: PathBuf,
}

impl Physical
{
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
}
