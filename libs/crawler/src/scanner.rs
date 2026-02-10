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

    todo!("Handle directories and recursion")
}
