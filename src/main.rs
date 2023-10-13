use clap::Parser;

#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf
}

fn main() {
    let _args = CLI::parse();

    println!("Hello, world!");
}
