use crawler::types::Node;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Binary,
}

pub fn save_scan(node: &Node, path: &Path, format: OutputFormat) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    match format {
        OutputFormat::Json => {
            serde_json::to_writer_pretty(&mut writer, node)?;
            writeln!(writer)?;
        }

        OutputFormat::Binary => {
            bincode::serialize_into(&mut writer, node).map_err(io::Error::other)?;
        }
    }

    writer.flush()?;

    Ok(())
}
