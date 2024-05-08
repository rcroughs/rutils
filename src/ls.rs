use std::{env, ffi::OsStr, fs::{self, Metadata}, io, os::{linux::fs::MetadataExt, unix::fs::PermissionsExt}, path::PathBuf, process::exit};
use nix::unistd::{Gid, Group, Uid, User};

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

fn show_metadata(metadata: &Metadata) {
    if metadata.is_dir() {
        print!("d")
    } else {
        print!("-")
    }
    let mode : u32 = metadata.permissions().mode();
    for i in (0..9).rev() {
        if mode & (1 << i) != 0 {
            match i % 3 {
                0 => print!("x"),
                1 => print!("w"),
                2 => print!("r"),
                _ => print!("-")
            }
        } else {
            print!("-")
        }
    }
    print!(" ");
}

fn show_user(metadata: &Metadata) {
    let user_id: u32 = metadata.st_uid();
    let group_id: u32 = metadata.st_gid();
    let user : User = User::from_uid(Uid::from_raw(user_id)).unwrap().unwrap();
    let group : Group = Group::from_gid(Gid::from_raw(group_id)).unwrap().unwrap();
    print!("{} {} ", user.name, group.name);
}

fn show_file_size(metadata: &Metadata) {
    print!("{} ", metadata.st_size().to_string());
}

fn show_elements(elements: Vec<PathBuf>, all: bool, list: bool) {
    for element in elements {
        if !element.file_name().unwrap().to_str().unwrap().starts_with(".") || all {
            if list {
                let metadata: Metadata = element.metadata().unwrap();
                show_metadata(&metadata);
                show_user(&metadata);
                show_file_size(&metadata);
            }
            if element.metadata().unwrap().is_dir() {
                print!("\x1b[94m");
            }
            let s : &OsStr = element.file_name().unwrap();
            print!("{}", s.to_str().unwrap());
            print!("\x1b[0m  ");
            if list { println!() };
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