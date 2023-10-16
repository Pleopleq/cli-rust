#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CLI::parse();
    let file_content = File::open(&args.path);
    let content = match file_content {
        Ok(file) => file,
        Err(error) => {
            return Err(error.into());
        }
    };

    let mut reader = BufReader::new(content);
    let mut lines = String::new();
    reader.read_to_string(&mut lines);

    println!("pattern: {}, path: {}", args.pattern, args.path.display());

    for line in lines.lines() {
        if line.contains(&args.pattern) {
            println!("Result: {}", line);
        }
    }

    Ok(())
}
