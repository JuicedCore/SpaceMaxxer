use crate::types::{Metadata, Node, NodeKind};
use rayon::prelude::*;
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
                    modified: None,
                },
                kind: NodeKind::File,
            };
        }
    };

    let modified = os_metadata.modified().ok();
    //Handling files
    if os_metadata.is_file() || os_metadata.file_type().is_symlink() {
        return Node {
            metadata: Metadata {
                name,
                path: path.to_path_buf(),
                size: os_metadata.len(),
                modified,
            },
            kind: NodeKind::File,
        };
    }

    //Handling folders parallely
    let mut children: Vec<Box<Node>> = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        let valid_entries: Vec<_> = entries.flatten().collect();
        children = valid_entries
            .into_par_iter()
            .map(|entry| {
                let path = entry.path();
                let node = scan(&path);
                Box::new(node)
            })
            .collect();
    }
    let total_size = children.iter().map(|c| c.metadata.size).sum();

    Node {
        metadata: Metadata {
            name,
            path: path.to_path_buf(),
            size: total_size,
            modified,
        },
        kind: NodeKind::Directory(children),
    }
}
