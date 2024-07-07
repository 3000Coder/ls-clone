use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use std::path::PathBuf;

const SHOW_HIDDEN: bool = false;

// ? Maybe it is better idea to handle this with structs
enum DirectoryEntery {
    HiddenDirectory(String),
    HiddenFile(String),
    Directory(String),
    File(String),
}

fn is_hidden(s: &str) -> bool {
    if s.len() > 0 {
        if s.chars().next().unwrap() == '.' {
            return true;
        }
    }
    return false;
}

// ! Hidden files are stripped of leading dot, needs to be readded later
fn parse_pathbuffs(files: &Vec<PathBuf>) -> Vec<DirectoryEntery> {
    let mut res_buf: Vec<DirectoryEntery> = Vec::new();

    for file in files {
        if let Some(f) = file.file_name() {
            if let Some(filename) = f.to_str() {
                match (is_hidden(filename), file.is_file()) {
                    (false, false) => {
                        res_buf.push(DirectoryEntery::Directory(filename.to_string()));
                    }
                    (false, true) => {
                        res_buf.push(DirectoryEntery::File(filename.to_string()));
                    }
                    (true, false) => {
                        let mut filename: String = filename.to_string();
                        filename.remove(0);
                        res_buf.push(DirectoryEntery::HiddenDirectory(filename));
                    }
                    (true, true) => {
                        let mut filename: String = filename.to_string();
                        filename.remove(0);
                        res_buf.push(DirectoryEntery::HiddenFile(filename));
                    }
                }
            }
        }
    }

    return res_buf;
}

fn get_entry_name(entry: &DirectoryEntery) -> String {
    match entry {
        DirectoryEntery::Directory(n)
        | DirectoryEntery::HiddenDirectory(n)
        | DirectoryEntery::File(n)
        | DirectoryEntery::HiddenFile(n) => {
            return n.to_string()
        }
    }
}

fn main() -> io::Result<()> {
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut print_buf: String = String::new();

    entries.sort();
    let mut p_enteries: Vec<DirectoryEntery> = parse_pathbuffs(&entries);
    
    p_enteries.sort_by_key(|a| get_entry_name(&a));

    for name in p_enteries {
        match name {
            DirectoryEntery::HiddenDirectory(n) => {
                if SHOW_HIDDEN {
                    print_buf += &format!("\x1b[1;34m.{}\x1b[0m  ", n).to_string();
                }
            }
            DirectoryEntery::Directory(n) => {
                print_buf += &format!("\x1b[1;34m{}\x1b[0m  ", n).to_string();
            }
            DirectoryEntery::HiddenFile(n) => {
                if SHOW_HIDDEN {
                    print_buf += &format!(".{}  ", n).to_string();
                }
            }
            DirectoryEntery::File(n) => {
                print_buf += &format!("{}  ", n).to_string();
            }
        }
    }

    println!("{}", print_buf);

    Ok(())
}
