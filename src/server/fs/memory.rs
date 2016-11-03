use Error;
use server::FileSystem;

use std::path::Path;

const ROOT_DIR_NAME: &'static str = "";

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
    nodes: Vec<Node>,
}

impl Memory
{
    pub fn new() -> Self {
        Memory {
            root: Node {
                name: ROOT_DIR_NAME.to_owned(),
                kind: NodeKind::Directory(Directory { nodes: Vec::new() })
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

            for node in dir.nodes.iter() { if let Some(node) = node.find_node(child_parts.clone()) {
                return Some(node)
            }}
        }
        None
    }

    fn find_node_mut(&mut self, parts: Vec<&str>) -> Option<&mut Self> {
        if parts == vec![&self.name] { return Some(self) };

        if let NodeKind::Directory(ref mut dir) = self.kind {
            let child_parts = parts[1..].to_owned();

            for node in dir.nodes.iter_mut() { if let Some(node) = node.find_node_mut(child_parts.clone()) {
                return Some(node)
            }}
        }
        None
    }
}

impl FileSystem for Memory
{
    fn list(&self, path: &Path)
        -> Result<Vec<String>, Error> {
        let parent_node = self.find_node(path)?;

        match parent_node.kind {
            NodeKind::Directory(ref dir) => {
                Ok(dir.nodes.iter().map(|node| node.name.clone()).collect())
            },
            NodeKind::File(..) => panic!("this is not a directory"),
        }
    }

    fn mkdir(&mut self, parent: &Path, name: String) -> Result<(), Error> {
        let parent = self.find_node_mut(parent)?;

        if let NodeKind::Directory(ref mut dir) = parent.kind {
            dir.nodes.push(Node {
                name: name,
                kind: NodeKind::Directory(Directory { nodes: Vec::new() }),
            });
            Ok(())
        } else {
            panic!("not a dir")
        }
    }

    fn write(&mut self, path: &Path, data: Vec<u8>) -> Result<(), Error> {
        let parent = self.find_node_mut(parent)?;
        unimplemented!();
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

            fs.root_dir_mut().nodes.push(foo.clone());

            assert_eq!(fs.find_node(&Path::new("/foo")).unwrap(), &foo);
        }

        #[test]
        fn correctly_finds_top_level_directory() {
            let mut fs = Memory::new();
            let bar = Node {
                name: "bar".to_owned(),
                kind: NodeKind::Directory(Directory { nodes: Vec::new() }),
            };

            fs.root_dir_mut().nodes.push(bar.clone());

            assert_eq!(fs.find_node(&Path::new("/bar")).unwrap(), &bar);
        }

        #[test]
        fn correctly_finds_nested_file() {
            let mut fs = Memory::new();
            let foo = Node {
                name: "foo".to_owned(),
                kind: NodeKind::Directory(Directory { nodes: Vec::new() }),
            };

            let bar = Node {
                name: "bar".to_owned(),
                kind: NodeKind::Directory(Directory {
                    nodes: vec![foo.clone()],
                }),
            };

            fs.root_dir_mut().nodes.push(bar.clone());

            assert_eq!(fs.find_node(&Path::new("/bar/foo")).unwrap(), &foo);
        }
    }

    mod mkdir {
        use super::super::{Node, NodeKind, Directory};
        pub use super::*;
        use server::FileSystem;
        use std::path::Path;

        #[test]
        fn correctly_creates_a_top_level_dir() {
            let mut fs = Memory::new();
            fs.mkdir(&Path::new("/"), "bar".to_string()).unwrap();

            assert_eq!(fs.root, Node {
                name: super::super::ROOT_DIR_NAME.to_owned(),
                kind: NodeKind::Directory(Directory{
                    nodes: vec![Node {
                        name: "bar".to_owned(),
                        kind: NodeKind::Directory(Directory { nodes: Vec::new() }),
                    }],
                }),
            });
        }
    }
}
