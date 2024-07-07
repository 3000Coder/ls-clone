use std::fs::{self};
use std::io;
use std::path::PathBuf;

const SHOW_HIDDEN: bool = true;

enum DirectoryEntery {
    Directory { name: String, is_hidden: bool },
    File { name: String, is_hidden: bool },
}

fn is_hidden(s: &str) -> bool {
    if s.len() > 0 {
        if s.chars().next().unwrap() == '.' {
            return true;
        }
    }
    return false;
}

fn parse_pathbuffs(files: &Vec<PathBuf>) -> Vec<DirectoryEntery> {
    let mut res_buf: Vec<DirectoryEntery> = Vec::new();

    for file in files {
        if let Some(f) = file.file_name() {
            if let Some(filename) = f.to_str() {
                if file.is_file() {
                    res_buf.push(DirectoryEntery::File {
                        name: filename.to_string(),
                        is_hidden: is_hidden(filename),
                    })
                } else {
                    res_buf.push(DirectoryEntery::Directory {
                        name: filename.to_string(),
                        is_hidden: is_hidden(filename),
                    })
                }
            }
        }
    }

    return res_buf;
}

fn get_entry_name_without_dot(entry: &DirectoryEntery) -> String {
    match entry {
        DirectoryEntery::File {
            name: n,
            is_hidden: hidden,
        }
        | DirectoryEntery::Directory {
            name: n,
            is_hidden: hidden,
        } => {
            if *hidden {
                let mut chars = n.chars();
                chars.next();
                chars.next_back();
                return chars.as_str().to_string();
            } else {
                return n.to_string();
            }
        }
    }
}

fn main() -> io::Result<()> {
    let entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut print_buf: String = String::new();

    let mut p_enteries: Vec<DirectoryEntery> = parse_pathbuffs(&entries);

    if SHOW_HIDDEN {
        p_enteries.push(DirectoryEntery::Directory {
            name: String::from("."),
            is_hidden: true,
        });
        p_enteries.push(DirectoryEntery::Directory {
            name: String::from(".."),
            is_hidden: true,
        });
    }

    p_enteries.sort_by_key(|a| get_entry_name_without_dot(&a));

    for entry in p_enteries {
        match entry {
            DirectoryEntery::Directory {
                name: n,
                is_hidden: h,
            } => {
                if !h || SHOW_HIDDEN {
                    print_buf += &format!("\x1b[1;34m{}\x1b[0m  ", n).to_string();
                }
            }
            DirectoryEntery::File {
                name: n,
                is_hidden: h,
            } => {
                if !h || SHOW_HIDDEN {
                    print_buf += &format!("{}  ", n).to_string();
                }
            }
        }
    }

    println!("{}", print_buf);

    Ok(())
}
