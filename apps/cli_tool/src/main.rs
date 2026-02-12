use clap::Parser;
use crawler::scanner::scan;
use rfd::FileDialog;
use std::path::PathBuf;
use std::time::Instant;
use storage::{OutputFormat, save_scan};

#[derive(Parser, Debug)]
#[command(name = "Spacemaxxer CLI", version, about = "Disk space analysis EZ")]
struct Args {
    //The input path name bruhhh
    path: Option<PathBuf>,
    //The  output file name
    output: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    json: bool,
}

fn main() {
    let args = Args::parse();
    let (format, extension) = if args.json {
        (OutputFormat::Json, "json")
    } else {
        (OutputFormat::Binary, "bin")
    };
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
                .set_file_name(format!("Disk_map.{}", extension))
                .add_filter("SpaceMaxxer Data", &[extension])
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

    match save_scan(&root_node, &save_path, format) {
        Ok(_) => {
            println!(
                "Your Potential has been analzyed and saved to {:?}",
                save_path
            );
            println!(
                "Total Potential the LORDS see: {:?}  bytes!",
                root_node.metadata.size
            );

            println!(" 🕐 The total time taken is: {:?}", duration);
        }
        Err(e) => {
            eprintln!("Data failed to save CHUD, The Error was: {}", e);
        }
    }
}
