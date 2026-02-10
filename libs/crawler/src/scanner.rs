use crate::types::{Metadata, Node, NodeKind};
use std::fs;
use std::path::Path;

pub fn scan(path: &Path) -> Node {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "/".to_string());
    // getting metadata here (we consider symlinks as leaf nodes ) if Err, we return a zero-size
    // file
    let os_metadata = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => {
            return Node {
                metadata: Metadata {
                    name,
                    path: path.to_path_buf(),
                    size: 0,
                },
                kind: NodeKind::File,
            };
        }
    };
    //Handling files
    if os_metadata.is_file() || os_metadata.file_type().is_symlink() {
        return Node {
            metadata: Metadata {
                name,
                path: path.to_path_buf(),
                size: os_metadata.len(),
            },
            kind: NodeKind::File,
        };
    }
    //Handling folders

    let mut children = Vec::new();
    let mut total_size = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(|e| e.ok()) {
            //calling scan on the child path
            let child_node = scan(&entry.path());
            //adding to size of the parent Node
            total_size += child_node.metadata.size;
            //pushin the child to p (Parent) [Pushin P lmaooo]
            children.push(Box::new(child_node));
        }
    }

    Node {
        metadata: Metadata {
            name,
            path: path.to_path_buf(),
            size: total_size,
        },
        kind: NodeKind::Directory(children),
    }
}
