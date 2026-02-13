use crate::{Block, LayoutConfig};
use crawler::types::{Node, NodeKind};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

pub fn build_island(
    node: &Node,
    x: f32,
    z: f32,
    w: f32,
    d: f32,
    layer: u32,
    config: &LayoutConfig,
) -> Vec<Block> {
    let mut blocks = Vec::new();

    if node.metadata.size < config.min_size {
        return blocks;
    }

    let (is_dir, children_ref) = match &node.kind {
        NodeKind::Directory(c) => (true, Some(c)),
        NodeKind::File => (false, None),
    };

    let platform_y = layer as f32 * config.floor_height;

    blocks.push(Block {
        x,
        y: platform_y,
        z,
        width: w,
        height: config.floor_height,
        depth: d,
        name: node.metadata.name.clone(),
        path: node.metadata.path.clone(),
        size: node.metadata.size,
        is_dir,
        layer,
        color_hash: hash_string(&node.metadata.name),
    });
    let children = match children_ref {
        Some(c) if !c.is_empty() => c,
        _ => return blocks,
    };

    let padding = config.get_padding(layer);
    let inner_x = x + padding;
    let inner_z = z + padding;
    let inner_w = (w - padding * 2.0).max(0.0);
    let inner_d = (d - padding * 2.0).max(0.0);

    if inner_w < 0.0 || inner_d < 0.0 {
        return blocks;
    }

    let visible_children: Vec<&Node> = children
        .iter()
        .map(|boxed| &**boxed)
        .filter(|c| c.metadata.size >= config.min_size)
        .collect();

    let total_size: u64 = visible_children.iter().map(|c| c.metadata.size).sum();

    if total_size == 0 {
        return blocks;
    }

    let mut current_child_x = inner_x;

    for child in visible_children {
        let share = child.metadata.size as f64 / total_size as f64;
        let child_w = (inner_w as f64 * share) as f32;

        if matches!(child.kind, NodeKind::Directory(_)) {
            let mut child_blocks = build_island(
                child,
                current_child_x,
                inner_z,
                child_w,
                inner_d,
                layer + 1,
                config,
            );
            blocks.append(&mut child_blocks);
        } else {
            blocks.push(
                create_file_block(child, current_child_x, inner_z, layer + 1, config)
                    .with_dims(child_w, inner_d),
            );
        }

        current_child_x += child_w;
    }
    blocks
}

pub fn create_file_block(node: &Node, x: f32, z: f32, layer: u32, config: &LayoutConfig) -> Block {
    let log_size = (node.metadata.size as f64).log10().max(1.0);
    let height = (log_size * 2.0) as f32;

    Block {
        x,
        y: layer as f32 * config.floor_height,
        z,
        width: 1.0,
        height,
        depth: 1.0,
        name: node.metadata.name.clone(),
        path: node.metadata.path.clone(),
        size: node.metadata.size,
        is_dir: false,
        layer,
        color_hash: hash_string(&node.metadata.name),
    }
}

impl Block {
    pub fn with_dims(mut self, w: f32, d: f32) -> Self {
        self.width = w;
        self.depth = d;
        self
    }
}
