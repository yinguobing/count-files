use clap::Parser;
use count_files::{run, Args};
use std::process;

fn main() {
    env_logger::init();
    let args = Args::parse();

    if let Err(e) = run(&args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
