use std::env;
use std::fs;
use std::process::exit;

fn main() {
    for path in env::args().skip(1) {
        println!("{}",fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("{e}");
            exit(1);
        }));
    }
}