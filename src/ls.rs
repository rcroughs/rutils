use std::{env, ffi::OsStr, fs, io, path::PathBuf, process::exit, str::FromStr};

struct Params {
    list: bool,
    all: bool
}

fn directory_elements(path: PathBuf) -> io::Result<Vec<PathBuf>> {
    let dir = fs::read_dir(path)?;
    let res : Vec<PathBuf> = dir.filter_map(|elem| Some(elem.ok()?.path())).collect();
    Ok(res)
}

fn parse_args() -> Params {
    let mut params : Params = Params {
        list: false,
        all: false,
    };
    for arg in env::args().skip(1) {
        if arg.starts_with("-") {
            if arg.contains("l") {
                params.list = true;
            }
            if arg.contains("a") {
                params.all = true;
            }
        }
    }
    return params;
}

fn show_elements(elements: Vec<PathBuf>, all: bool, list: bool) {
    for element in elements {
        if !element.file_name().unwrap().to_str().unwrap().starts_with(".") || all {
            if element.metadata().unwrap().is_dir() {
                print!("\x1b[94m");
            }
            let s : &OsStr = element.file_name().unwrap();
            print!("{}", s.to_str().unwrap());
            print!("\x1b[0m  ");
        }
        if list {
            println!();
        }
    }
    print!("\n")
}

fn main() {
    let params : Params = parse_args();
    let path : PathBuf = env::current_dir().unwrap_or_else(|e| {
        eprintln!("{e}");
        exit(1);
    });
    show_elements(directory_elements(path).unwrap(), params.all, params.list);
}