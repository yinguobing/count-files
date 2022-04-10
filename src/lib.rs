use comfy_table::Table;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

// Config struct converts the user inputs into arguments.
pub struct Config {
    pub target_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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
        Ok(Self {
            target_path: target_path,
        })
    }
}

// Scan the target path and count all the files.
fn scan(path: &Path, counter: &mut HashMap<String, usize>) -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Err(e) = scan(path.as_path(), counter) {
                println!("WARNING: {}. Skip {}", e, path.to_str().unwrap());
            }
        } else {
            if let Some(extension) = path.extension() {
                let extension = extension.to_str().unwrap().to_string();
                let count = counter.entry(extension).or_insert(0);
                *count += 1;
            }
        }
    }
    Ok(())
}

// Print the counting result.
fn print_to_screen(counter: &HashMap<String, usize>) {
    // Sort the result by file count.
    let mut counter: Vec<(&String, &usize)> = counter.iter().collect();
    counter.sort_by(|a, b| b.1.cmp(a.1));
    
    // Insert table rows.
    let mut table = Table::new();
    table.set_header(vec!["File type", "Count"]);
    for (ext, count) in counter {
        table.add_row(vec![ext, &count.to_string()]);
    }

    // Align the numbers to right.
    if let Some(column) = table.get_column_mut(1) {
        column.set_cell_alignment(comfy_table::CellAlignment::Right)
    }
    println!("{table}");
}

// Count all the files in this function.
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // Use a hashmap to record different files count.
    let mut counter: HashMap<String, usize> = HashMap::new();
    let target_path = Path::new(&config.target_path);
    scan(&target_path, &mut counter)?;
    println!("Counting files in {}", config.target_path);
    print_to_screen(&counter);
    Ok(())
}
