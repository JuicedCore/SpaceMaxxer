use crawler::types::{Node, NodeKind};
use rfd::FileDialog;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use treemap::{Rect, squarify};

// A helper struct to hold the final data for drawing
struct DrawItem {
    rect: Rect,
    name: String,
    depth: usize,
    is_dir: bool,
}

fn main() {
    // 1. Check for 'disk_map.json', otherwise open File Dialog
    let file_path = if Path::new("disk_map.json").exists() {
        println!("Found 'disk_map.json' automatically.");
        PathBuf::from("disk_map.json")
    } else {
        println!("'disk_map.json' not found in root. Opening file picker...");
        match FileDialog::new()
            .add_filter("JSON Data", &["json"])
            .set_directory(".")
            .pick_file()
        {
            Some(path) => path,
            None => {
                eprintln!("No file selected. Exiting.");
                return;
            }
        }
    };

    println!("Loading data from: {:?}", file_path);

    // 2. Load the Data
    let file = File::open(&file_path).expect("Could not open file");
    let reader = BufReader::new(file);
    let root: Node = serde_json::from_reader(reader).expect("JSON was malformed or empty");

    // 3. Setup the Canvas (1000x1000)
    let canvas = Rect {
        x: 0.0,
        y: 0.0,
        w: 1000.0,
        h: 1000.0,
    };
    let mut draw_list = Vec::new();

    println!("Calculating layout for project: {}", root.metadata.name);

    // 4. Recursive Layout Calculation
    calculate_layout(&root, canvas, 0, &mut draw_list);

    // 5. Save to SVG
    save_svg(&draw_list, "map.svg");
}

fn calculate_layout(node: &Node, area: Rect, depth: usize, list: &mut Vec<DrawItem>) {
    // Add current node to the draw list
    list.push(DrawItem {
        rect: area,
        name: node.metadata.name.clone(),
        depth,
        is_dir: matches!(node.kind, NodeKind::Directory(_)),
    });

    // If it's a folder, we need to layout its children inside it
    if let NodeKind::Directory(children) = &node.kind {
        if children.is_empty() {
            return;
        }

        // CRITICAL: Your squarify library sorts items by size (Largest -> Smallest).
        // To make sure 'Rect 1' belongs to 'Child 1', we MUST sort our children
        // to match the order your library expects.
        //
        //
        let mut sorted_children: Vec<&Node> = children.iter().map(|b| b.as_ref()).collect();
        sorted_children.sort_by(|a, b| b.metadata.size.cmp(&a.metadata.size));

        // 1. Extract weights (File Sizes)
        let weights: Vec<f64> = sorted_children
            .iter()
            .map(|c| c.metadata.size as f64)
            .collect();

        // 2. Run the Algorithm
        // Note: Your algorithm expects the outer container, so we pass 'area'
        let child_rects = squarify(area, &weights);

        // 3. Map the result rectangles back to the children
        for (i, rect) in child_rects.into_iter().enumerate() {
            // Safety check in case weights/rects count mismatch
            if let Some(child_node) = sorted_children.get(i) {
                calculate_layout(child_node, rect, depth + 1, list);
            }
        }
    }
}

fn save_svg(items: &[DrawItem], filename: &str) {
    // Basic SVG Header
    let mut svg = String::from(
        r#"<svg viewBox="0 0 1000 1000" xmlns="http://www.w3.org/2000/svg" style="background:#111;">"#,
    );

    for item in items {
        // Color Logic
        let fill = if item.is_dir {
            // Folders: Dark Blue/Grey, getting lighter with depth
            // Depth 0 = 10% Lightness, Depth 5 = 35% Lightness
            let l = (10 + item.depth * 5).min(50);
            format!("hsl(220, 30%, {}%)", l)
        } else {
            // Files: Colorful based on name hash
            let hash: usize = item.name.bytes().map(|b| b as usize).sum();
            format!("hsl({}, 70%, 60%)", hash % 360)
        };

        let stroke = "#000";
        // Thicker stroke for folders to distinguish hierarchy
        let stroke_w = if item.is_dir { "2.0" } else { "0.5" };

        // Draw Rect
        svg.push_str(&format!(
            r#"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" fill="{}" stroke="{}" stroke-width="{}" />"#,
            item.rect.x, item.rect.y, item.rect.w, item.rect.h, fill, stroke, stroke_w
        ));

        // Draw Text (Only if box is big enough to be legible)
        if item.rect.w > 40.0 && item.rect.h > 20.0 {
            svg.push_str(&format!(
                r#"<text x="{:.2}" y="{:.2}" fill="white" font-size="12" font-family="Arial" pointer-events="none">{}</text>"#,
                item.rect.x + 5.0, item.rect.y + 15.0, item.name
            ));
        }
    }

    svg.push_str("</svg>");

    let mut f = File::create(filename).expect("Could not create SVG file");
    f.write_all(svg.as_bytes())
        .expect("Could not write to SVG file");
    println!("SUCCESS: 2D Treemap saved to {}", filename);
}
