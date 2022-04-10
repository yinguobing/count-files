use std::env;
use std::path::Path;
use std::process;
use std::fs;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

// Config struct converts the user inputs into arguments.
struct Config {
    target_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // Target dir provided?
        if args.len() < 2 {
            return Err(
                "target directory not assigned.\nTry running like this:\n   count_files /path/to/dir",
            );
        }
        // Target dir exist?
        let target_path = args[1].to_string();
        if !Path::new(&target_path).exists() {
            return Err("target directory not found.");
        }
        // All is well.
        Ok(Self {target_path: target_path})
    }
}

// Count all the files in this function.
fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("Counting files in {}", config.target_path);
    let entries = fs::read_dir(config.target_path.clone())?;
    Ok(())
}