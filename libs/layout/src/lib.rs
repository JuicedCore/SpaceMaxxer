pub mod packer;
pub mod treemap;

use crawler::types::{Node, NodeKind};
use std::path::PathBuf;

#[derive(Debug, Clone)]

pub struct Block {
    //Geometry
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    pub depth: f32,

    //identity
    pub name: String,
    pub path: PathBuf,
    pub size: u64,

    //Metadata
    pub is_dir: bool,
    pub layer: u32,
    pub color_hash: u64,
}

pub struct LayoutConfig {
    //file size
    pub min_size: u64,

    //Area per byte
    pub scale_factor: f64,

    pub island_gap: f32,

    //floor thickness for folders
    pub floor_height: f32,

    pub layer_padding: Vec<f32>,

    pub default_padding: f32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            min_size: 1024 * 10,
            scale_factor: 0.0001,
            island_gap: 50.0,
            floor_height: 1.0,

            layer_padding: vec![10.0, 5.0],
            default_padding: 1.0,
        }
    }
}

impl LayoutConfig {
    pub fn get_padding(&self, layer: u32) -> f32 {
        *self
            .layer_padding
            .get(layer as usize)
            .unwrap_or(&self.default_padding)
    }
}

pub fn generate_layout(root: &Node, config: &LayoutConfig) -> Vec<Block> {
    let children = match &root.kind {
        NodeKind::Directory(c) => c,
        NodeKind::File => {
            // It's a file, just return one block
            return vec![treemap::create_file_block(root, 0.0, 0.0, 0, config)];
        }
    };

    let islands: Vec<&Node> = children.iter().map(|boxed_node| &**boxed_node).collect();

    let island_positions = packer::pack_islands(&islands, config);

    let mut all_blocks = Vec::new();

    for (node, x, z, w, d) in island_positions {
        let mut island_blocks = treemap::build_island(node, x, z, w, d, 0, config);
        all_blocks.append(&mut island_blocks);
    }

    all_blocks
}

// ... existing code ...

#[cfg(test)]
mod tests {
    use super::*;
    use crawler::types::{Metadata, NodeKind};
    use std::time::SystemTime;

    // Helper to create a fake node
    fn create_node(name: &str, size: u64, is_dir: bool, children: Vec<Node>) -> Node {
        Node {
            metadata: Metadata {
                name: name.to_string(),
                path: PathBuf::from(name),
                size,
                modified: Some(SystemTime::now()),
            },
            kind: if is_dir {
                // Box the children
                let boxed_children = children.into_iter().map(Box::new).collect();
                NodeKind::Directory(boxed_children)
            } else {
                NodeKind::File
            },
        }
    }

    #[test]
    fn test_layout_generation() {
        // 1. Setup a fake file system
        // Root ->
        //   island_A (100 MB) -> file_1 (50 MB), file_2 (50 MB)
        //   island_B (10 MB) -> file_3 (10 MB)

        let file_1 = create_node("file_1.txt", 50 * 1024 * 1024, false, vec![]);
        let file_2 = create_node("file_2.txt", 50 * 1024 * 1024, false, vec![]);

        let island_a = create_node("island_A", 100 * 1024 * 1024, true, vec![file_1, file_2]);

        let file_3 = create_node("file_3.txt", 10 * 1024 * 1024, false, vec![]);

        let island_b = create_node("island_B", 10 * 1024 * 1024, true, vec![file_3]);

        let root = create_node("root", 110 * 1024 * 1024, true, vec![island_a, island_b]);

        // 2. Run Layout
        let config = LayoutConfig::default();
        let blocks = generate_layout(&root, &config);

        // 3. Print Results (Use 'cargo test -- --nocapture' to see this)
        println!("Generated {} blocks:", blocks.len());
        for block in &blocks {
            println!(
                "[{}] name='{}' pos=({:.1}, {:.1}, {:.1}) size={:.1}x{:.1}x{:.1}",
                if block.is_dir { "DIR " } else { "FILE" },
                block.name,
                block.x,
                block.y,
                block.z,
                block.width,
                block.height,
                block.depth
            );
        }

        // 4. Assertions
        assert!(blocks.len() > 0, "Should generate blocks");

        // Find our islands
        let island_a_block = blocks
            .iter()
            .find(|b| b.name == "island_A")
            .expect("Missing island_A");
        let island_b_block = blocks
            .iter()
            .find(|b| b.name == "island_B")
            .expect("Missing island_B");

        // Verify packing: Island B should be shifted along X axis (Shelf Packing)
        // Island A is huge (100MB), so it starts at 0.
        // Island B should start after A ends + gap.
        assert!(
            island_b_block.x > island_a_block.x,
            "Island B should be to the right of Island A"
        );

        // Verify stacking: Files should be higher than folders
        let file_block = blocks
            .iter()
            .find(|b| b.name == "file_1.txt")
            .expect("Missing file_1");
        assert!(
            file_block.y > island_a_block.y,
            "File should be stacked ON TOP of the folder platform"
        );
    }
}
