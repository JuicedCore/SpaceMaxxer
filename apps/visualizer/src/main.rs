use crawler::scanner::scan;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let scan_path = Path::new(".");
    println!("SpaceMaxxer scanning {:?}", scan_path);

    let root_node = scan(scan_path);

    let json_data = serde_json::to_string_pretty(&root_node).expect("Failed to serialize disk map");

    let mut file = File::create("disk_map.json").expect("Failed to create file");

    file.write_all(json_data.as_bytes())
        .expect("failed to write to file");

    println!("Done GGWP bro");
}
