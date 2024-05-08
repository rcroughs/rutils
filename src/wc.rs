use core::panic;
use std::env;
use std::process::exit;
use std::fs;

struct WordCount {
    chars: u128,
    words: u128,
    lines: u128
}

fn count_buf(buf: &String) -> WordCount {
    let mut wc: WordCount = WordCount {
        chars: 0,
        words: 0,
        lines: 0
    };
    let mut in_word: bool = false;
    for char in buf.chars() {
        if char == '\n' {
            wc.lines += 1;
        } 
        if char == ' ' || char == '\t' || char == '\n' {
            in_word = false;
        } else {
            if !in_word {
                wc.words += 1;
                in_word = true;
            }
        }
        wc.chars += 1;
    };
    wc
}

fn main() {
    let path: String = env::args()
                            .nth(1)
                            .unwrap_or_else(|| { 
                                eprintln!("Please do include a file"); 
                                exit(1)
                            });
    let buf: String = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => panic!("Couldn't find this file")
    };
    let wc: WordCount =  count_buf(&buf);
    println!(" {} {} {} {}", wc.lines.to_string(), wc.words.to_string(), wc.chars.to_string(), path);
}