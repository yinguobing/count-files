use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dirname = parse_config(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Counting files in {:?}", dirname);
    
}

fn parse_config(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("target directory not assigned.\nTry running like this:\n   count_files /path/to/dir");
    }
    let target_path = args[1].to_string();
    Ok(target_path)
}