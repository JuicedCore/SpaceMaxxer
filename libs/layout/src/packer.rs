use crate::LayoutConfig;
use crawler::types::Node;

/// Arranges top-level nodes into a grid/shelf layout.
/// Returns a list of (Node Reference, X, Z, Width, Depth).
pub fn pack_islands<'a>(
    nodes: &'a [&'a Node], // Input: Slice of references
    config: &LayoutConfig,
) -> Vec<(&'a Node, f32, f32, f32, f32)> {
    // Output: Vec of references
    let mut placed_items = Vec::new();

    // 1. Sort by size
    // We clone the *references*, not the Nodes themselves.
    // This gives us a new Vec<&Node> we can sort.
    let mut sorted_nodes = nodes.to_vec();
    sorted_nodes.sort_by(|a, b| b.metadata.size.cmp(&a.metadata.size));

    let mut current_x = 0.0;
    let mut current_z = 0.0;
    let mut row_max_depth: f32 = 0.0;
    let max_row_width = 1000.0;

    for node in sorted_nodes {
        // Dereference once to get to the Node data for size check
        if node.metadata.size < config.min_size {
            continue;
        }

        let area = node.metadata.size as f64 * config.scale_factor;
        let side = area.sqrt() as f32;

        let width = side.max(5.0);
        let depth = side.max(5.0);

        if current_x + width > max_row_width {
            current_x = 0.0;
            current_z += row_max_depth + config.island_gap;
            row_max_depth = 0.0;
        }

        placed_items.push((node, current_x, current_z, width, depth));

        current_x += width + config.island_gap;
        row_max_depth = row_max_depth.max(depth);
    }

    placed_items
}
