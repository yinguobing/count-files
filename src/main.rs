use clap::Parser;
use std::process;

use count_files::{Args, run};

fn main() {
    let args = Args::parse();

    if let Err(e) = run(&args) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
