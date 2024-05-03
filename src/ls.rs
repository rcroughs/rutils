use std::{env, fs, io, path::PathBuf, str::FromStr};

fn directory_elements(path: String) -> io::Result<Vec<PathBuf>> {
    let dir = fs::read_dir(path)?;
    let res : Vec<PathBuf> = dir.filter_map(|elem| Some(elem.ok()?.path())).collect();
    Ok(res)
}

fn main() {
    let path : String = match env::args().nth(1) {
        Some(s) => s,
        None => String::from_str(".").unwrap()
    };
    let elements : Vec<PathBuf> = directory_elements(path).unwrap();

    for element in elements {
        let s : &str = element.to_str().unwrap();
        println!("{s}");
    }
}