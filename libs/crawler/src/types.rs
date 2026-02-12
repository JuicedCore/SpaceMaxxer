use serde::Serialize;
use std::{path::PathBuf, time::SystemTime};

#[derive(Debug, Clone, Serialize)]
pub struct Metadata {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub modified: Option<SystemTime>,
}

#[derive(Debug, Clone, Serialize)] // Add Serialize here
pub enum NodeKind {
    File,
    Directory(Vec<Box<Node>>),
}

#[derive(Debug, Clone, Serialize)] // Add Serialize here
pub struct Node {
    pub metadata: Metadata,
    pub kind: NodeKind,
}
