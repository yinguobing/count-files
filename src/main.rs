use std::env;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Counting files in {:?}", config.target_path);
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
