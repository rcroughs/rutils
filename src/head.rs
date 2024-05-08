use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let nbr_lines : usize = 10;
    env::args().skip(1).for_each(|path: String| {
        let file: File = File::open(&path).unwrap_or_else(|e| {
            panic!("{}", e)
        });
        let reader : BufReader<File> = BufReader::new(file);
        let lines : Vec<String> = reader.lines().take(nbr_lines).map(|line| match line {
            Ok(s) => s,
            Err(e) => panic!("{}", e)
        }).collect();
        if env::args().len() > 2 {
            println!("==> {} <==", &path);
        }
        lines.iter().for_each(|line| println!("{}", line));
    });
}