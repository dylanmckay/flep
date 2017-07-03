use Error;
use super::FileSystem;

use std::collections::HashMap;
use std::path::Path;

const ROOT_DIR_NAME: &'static str = "";

/// An in-memory filesystem.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Memory
{
    /// The root directory.
    root: Node,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node
{
    name: String,
    kind: NodeKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum NodeKind
{
    File(File),
    Directory(Directory),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct File
{
    data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Directory
{
    nodes: HashMap<String, Node>,
}

impl Memory
{
    pub fn new() -> Self {
        Memory {
            root: Node {
                name: ROOT_DIR_NAME.to_owned(),
                kind: NodeKind::Directory(Directory { nodes: HashMap::new() })
            },
        }
    }

    #[cfg(test)]
    fn root_dir_mut(&mut self) -> &mut Directory {
        if let NodeKind::Directory(ref mut dir) = self.root.kind { dir } else { unreachable!() }
    }

    fn find_node(&self, path: &Path) -> Result<&Node, Error> {
        let mut parts: Vec<_> = path.iter().map(|s| s.to_str().unwrap()).collect();

        // Skip the '/' if it exists.
        if parts.first() == Some(&"/") {
            parts = parts[1..].to_owned();
        }

        if parts.is_empty() {
            Ok(&self.root)
        } else {
            let mut the_parts = vec![ROOT_DIR_NAME];
            the_parts.extend(parts);
            let node = self.root.find_node(the_parts);

            if let Some(node) = node { Ok(node) } else { panic!("path does not exist") }
        }
    }

    fn find_node_mut(&mut self, path: &Path) -> Result<&mut Node, Error> {
        let mut parts: Vec<_> = path.iter().map(|s| s.to_str().unwrap()).collect();

        // Skip the '/' if it exists.
        if parts.first() == Some(&"/") {
            parts = parts[1..].to_owned();
        }

        if parts.is_empty() {
            Ok(&mut self.root)
        } else {
            let mut the_parts = vec![ROOT_DIR_NAME];
            the_parts.extend(parts);
            let node = self.root.find_node_mut(the_parts);

            if let Some(node) = node { Ok(node) } else { panic!("path does not exist") }
        }
    }
}

impl Node
{
    fn find_node(&self, parts: Vec<&str>) -> Option<&Self> {
        if parts == vec![&self.name] { return Some(self) };

        if let NodeKind::Directory(ref dir) = self.kind {
            let child_parts = parts[1..].to_owned();

            for node in dir.nodes.values() { if let Some(node) = node.find_node(child_parts.clone()) {
                return Some(node)
            }}
        }
        None
    }

    fn find_node_mut(&mut self, parts: Vec<&str>) -> Option<&mut Self> {
        if parts == vec![&self.name] { return Some(self) };

        if let NodeKind::Directory(ref mut dir) = self.kind {
            let child_parts = parts[1..].to_owned();

            for node in dir.nodes.values_mut() { if let Some(node) = node.find_node_mut(child_parts.clone()) {
                return Some(node)
            }}
        }
        None
    }
}

impl Directory
{
    #[cfg(test)]
    pub fn new() -> Self {
        Directory { nodes: HashMap::new() }
    }

    #[cfg(test)]
    pub fn add(mut self, node: Node) -> Self {
        self.nodes.insert(node.name.clone(), node);
        self
    }
}

impl FileSystem for Memory
{
    fn list(&self, path: &Path)
        -> Result<Vec<String>, Error> {
        let parent_node = self.find_node(path)?;

        match parent_node.kind {
            NodeKind::Directory(ref dir) => {
                Ok(dir.nodes.values().map(|node| node.name.clone()).collect())
            },
            NodeKind::File(..) => panic!("this is not a directory"),
        }
    }

    fn mkdir(&mut self, parent: &Path, name: String) -> Result<(), Error> {
        let parent = self.find_node_mut(parent)?;

        if let NodeKind::Directory(ref mut dir) = parent.kind {
            dir.nodes.insert(name.clone(), Node {
                name: name,
                kind: NodeKind::Directory(Directory { nodes: HashMap::new() }),
            });
            Ok(())
        } else {
            panic!("not a dir")
        }
    }

    fn write(&mut self, path: &Path, data: Vec<u8>) -> Result<(), Error> {
        let parent = self.find_node_mut(path.parent().unwrap())?;

        if let NodeKind::Directory(ref mut dir) = parent.kind {
            let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
            dir.nodes.insert(file_name.clone(), Node {
                name: file_name,
                kind: NodeKind::File(File { data: data }),
            });
            Ok(())
        } else {
            panic!("parent must be a directory");
        }
    }

    fn read(&self, path: &Path) -> Result<Vec<u8>, Error> {
        let parent = self.find_node(path.parent().unwrap())?;

        if let NodeKind::Directory(ref dir) = parent.kind {
            let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();

            if let Some(ref node) = dir.nodes.get(&file_name) {
                match node.kind {
                    NodeKind::File(ref file) => {
                        Ok(file.data.clone())
                    },
                    NodeKind::Directory(..) => {
                        panic!("cannot read a directory");
                    },
                }
            } else {
                panic!("file does not exist");
            }
        } else {
            panic!("parent must be a directory");
        }
    }
}

#[cfg(test)]
mod test
{
    pub use super::*;

    mod find_node {
        use super::super::{Node, NodeKind, File, Directory};
        pub use super::*;
        use std::path::Path;

        #[test]
        fn correctly_finds_empty_root() {
            let fs = Memory::new();
            assert_eq!(fs.find_node(&Path::new("/")).unwrap(), &fs.root);
        }

        #[test]
        fn correctly_finds_empty_string_as_root() {
            let fs = Memory::new();
            assert_eq!(fs.find_node(&Path::new("")).unwrap(), &fs.root);
        }

        #[test]
        fn correctly_finds_top_level_file() {
            let mut fs = Memory::new();
            let foo = Node {
                name: "foo".to_owned(),
                kind: NodeKind::File(File { data: vec![1,2,3] }),
            };

            fs.root_dir_mut().nodes.insert(foo.name.clone(), foo.clone());

            assert_eq!(fs.find_node(&Path::new("/foo")).unwrap(), &foo);
        }

        #[test]
        fn correctly_finds_top_level_directory() {
            let mut fs = Memory::new();
            let bar = Node {
                name: "bar".to_owned(),
                kind: NodeKind::Directory(Directory::new()),
            };

            fs.root_dir_mut().nodes.insert(bar.name.clone(), bar.clone());

            assert_eq!(fs.find_node(&Path::new("/bar")).unwrap(), &bar);
        }

        #[test]
        fn correctly_finds_nested_file() {
            let mut fs = Memory::new();
            let foo = Node {
                name: "foo".to_owned(),
                kind: NodeKind::Directory(Directory::new()),
            };

            let bar = Node {
                name: "bar".to_owned(),
                kind: NodeKind::Directory(Directory::new().add(foo.clone())),
            };

            fs.root_dir_mut().nodes.insert(bar.name.clone(), bar.clone());

            assert_eq!(fs.find_node(&Path::new("/bar/foo")).unwrap(), &foo);
        }
    }

    mod mkdir {
        use super::super::{Node, NodeKind, Directory, FileSystem};
        pub use super::*;
        use std::path::Path;

        #[test]
        fn correctly_creates_a_top_level_dir() {
            let mut fs = Memory::new();
            fs.mkdir(&Path::new("/"), "bar".to_string()).unwrap();

            assert_eq!(fs.root, Node {
                name: super::super::ROOT_DIR_NAME.to_owned(),
                kind: NodeKind::Directory(Directory::new().add(Node {
                    name: "bar".to_owned(),
                    kind: NodeKind::Directory(Directory::new()),
                })),
            });
        }
    }

    mod write {
        use super::super::{Node, NodeKind, File, Directory, FileSystem};
        pub use super::*;
        use std::path::Path;

        #[test]
        fn correctly_creates_a_top_level_file() {
            let mut fs = Memory::new();

            fs.write(&Path::new("/foo.txt"), vec![1,2,3]).unwrap();

            assert_eq!(fs.root, Node {
                name: super::super::ROOT_DIR_NAME.to_owned(),
                kind: NodeKind::Directory(Directory::new().add(Node {
                    name: "foo.txt".to_owned(),
                    kind: NodeKind::File(File { data: vec![1,2,3] }),
                })),
            });
        }
    }

    mod read {
        pub use super::*;
        use super::super::FileSystem;
        use std::path::Path;

        #[test]
        fn correctly_reads_a_top_level_file() {
            let mut fs = Memory::new();

            fs.write(&Path::new("/foo.txt"), vec![1,2,3]).unwrap();
            assert_eq!(fs.read(&Path::new("/foo.txt")).unwrap(), vec![1,2,3]);
        }
    }
}
