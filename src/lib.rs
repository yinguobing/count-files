use clap::Parser;
use comfy_table::Table;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;

/// Counting files in a directory.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Target directory to be scanned.
    pub target_path: String,
}

// A struct to record the file count number and the total storage size.
struct Counter {
    count: usize,
    storage_size: usize,
}

impl Counter {
    // Create a new Counter struct.
    pub fn new(count: usize, storage_size: usize) -> Counter {
        Counter {
            count,
            storage_size,
        }
    }

    // Update the counter.
    pub fn update(&mut self, count: i64, storage_size: i64) {
        self.count = (self.count as i64 + count) as usize;
        self.storage_size = (self.storage_size as i64 + storage_size) as usize;
    }
}

impl Args {
    pub fn new(args: &[String]) -> Result<Args, &'static str> {
        // Target dir provided?
        if args.len() < 2 {
            return Err(
                "target directory not assigned.\nTry running like this:\n   count_files /path/to/dir",
            );
        }
        // Target dir exist?
        let target_path = args[1].to_string();
        if !Path::new(&target_path).exists() {
            return Err("target not found.");
        }
        // Target is a directory?
        if !Path::new(&target_path).is_dir() {
            return Err("target is not a directory.");
        }
        // All is well.
        Ok(Self { target_path })
    }
}

// Scan the target path and count all the files.
fn scan(
    path: &Path,
    record: &mut HashMap<String, Counter>,
    pb: ProgressBar,
) -> Result<(), Box<dyn Error>> {
    // Tell the user where are we now.
    pb.set_message(path.to_str().unwrap().to_string());

    // Loop the entries.
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Err(e) = scan(path.as_path(), record, pb.clone()) {
                println!("WARNING: {}. Skip {}", e, path.to_str().unwrap());
            }
        } else if let Some(extension) = path.extension() {
            let extension = extension.to_str().unwrap().to_string();
            let counter = record
                .entry(extension)
                .or_insert_with(|| Counter::new(0, 0));
            counter.update(1, 0);
        }
    }
    Ok(())
}

// Print the counting result in a table.
fn print_to_screen(record: &HashMap<String, Counter>) {
    // Sort the result by file count.
    let mut record: Vec<(&String, &Counter)> = record.iter().collect();
    record.sort_by(|a, b| b.1.count.cmp(&a.1.count));

    // Create the result table.
    let mut table = Table::new();
    table.set_header(vec!["File type", "Count"]);
    for (ext, counter) in record {
        table.add_row(vec![ext, &counter.count.to_string()]);
    }

    // Align the numbers to right.
    if let Some(column) = table.get_column_mut(1) {
        column.set_cell_alignment(comfy_table::CellAlignment::Right)
    }
    println!("{table}");
}

// Count all the files.
pub fn run(config: &Args) -> Result<(), Box<dyn Error>> {
    // Use a hashmap to record different files count.
    let mut record: HashMap<String, Counter> = HashMap::new();
    let target_path = Path::new(&config.target_path);

    // Setup a progress bar.
    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{prefix:.bold.dim} {spinner} {wide_msg}");
    let pb = ProgressBar::new_spinner();
    pb.set_style(spinner_style);
    pb.set_prefix("Scanning ");

    // Setup a duration meter.
    let started = Instant::now();

    // Let the party begin.
    scan(target_path, &mut record, pb.clone())?;

    // Post process
    pb.finish_and_clear();
    println!("Done in {}.", HumanDuration(started.elapsed()));
    print_to_screen(&record);
    Ok(())
}
