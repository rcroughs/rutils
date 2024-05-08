use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {
    env::args().skip(1).rev().for_each(|path: String| {
        let file: File = File::open(path).unwrap_or_else(|e| {
            panic!("{}", e);
        });
        let reader: BufReader<File> = BufReader::new(file);
        
        let lines: Vec<String> = reader.lines().map(|elem| match elem {
            Ok(s) => s,
            Err(e) => panic!("{}", e)
        }).collect();    

        let _ = lines.iter().rev().for_each(|elem| println!("{}", elem));
    });
}