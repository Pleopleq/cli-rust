#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = CLI::parse();
    let file_content = File::open(&args.path).expect("Could not find file");
    let mut reader = BufReader::new(file_content);

    let mut content = String::new();
    reader
        .read_to_string(&mut content)
        .expect("Cannot read string");

    println!("pattern: {}, path: {}", args.pattern, args.path.display());

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("Result: {}", line);
        }
    }
}
