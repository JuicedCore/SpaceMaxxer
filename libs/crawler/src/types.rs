use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Metadata {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
}

pub enum NodeKind {
    File,
    Directory(Vec<Box<Node>>),
}

pub struct Node {
    pub metadata: Metadata,
    pub kind: NodeKind,
}
