use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let file_path : String = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("No path found");
        exit(1);
    });

    let buffer : String =  fs::read_to_string(file_path).unwrap_or_else(|e| {
        eprintln!("{e}");
        exit(1);
    });

    print!("{buffer}\n");
}