use std::{env, ffi::OsStr, fs::{self, Metadata, Permissions}, io, os::unix::fs::PermissionsExt, path::PathBuf, process::exit, str::FromStr};

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

fn show_metadata(metadata: Metadata) {
    if metadata.is_dir() {
        print!("d")
    } else {
        print!("-")
    }
    let mode : u32 = metadata.permissions().mode();
    for i in (0..9).rev() {
        if mode & (1 << i) != 0 {
            if i % 3 == 0 {
                print!("x")
            } else if i % 3 == 1 {
                print!("w")
            } else {
                print!("r")
            }
        } else {
            print!("-")
        }
    }
    print!(" ");
}

fn show_elements(elements: Vec<PathBuf>, all: bool, list: bool) {
    for element in elements {
        if !element.file_name().unwrap().to_str().unwrap().starts_with(".") || all {
            if list {
                show_metadata(element.metadata().unwrap());
            }
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