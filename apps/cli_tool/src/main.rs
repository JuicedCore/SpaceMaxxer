use clap::Parser;
use crawler::scanner::scan;
use rfd::FileDialog;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(name = "Spacemaxxer CLI", version, about = "Disk space analysis EZ")]
struct Args {
    //The input path name bruhhh
    path: Option<PathBuf>,
    //The  output file name
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let target = match args.path {
        Some(p) => p,
        None => {
            println!("Select the directory to analyze its potetntial");
            FileDialog::new()
                .pick_folder()
                .expect("No folder selected, prolly a CHUD. Bye!")
        }
    };

    println!(
        "SpaceMaxxer is analyzing you frame mogging potential at {:?} ",
        target
    );
    let start_time = Instant::now();
    let root_node = scan(&target);

    let duration = start_time.elapsed();
    let save_path = match args.output {
        Some(p) => p,
        None => {
            println!("Choose location to save the output file brother!");
            FileDialog::new()
                .set_file_name("Disk_map.json")
                .add_filter("JSON", &["json"])
                .save_file()
                .expect("Stop cancelling the scan you CHUD")
        }
    };
    //
    //
    //
    //
    //
    //"The error mesages might be misleading in some cases like failed to create is used initially then I print succesfully generated etc, so might have to refactor this later.
    //
    //
    //

    let json = serde_json::to_string_pretty(&root_node).expect("Failed to serialize");
    let mut file = File::create(&save_path).expect("Failed to create file");
    file.write_all(json.as_bytes())
        .expect("Failed to write data");

    println!(
        "Your Potential has been analzyed and saved to {:?}",
        save_path
    );
    println!(
        "Total Potential I see: {:?}  bytes!",
        root_node.metadata.size
    );

    println!(" 🕐 The total time taken is: {:?}", duration);
}
