use std::fs::{self};
use std::io;
use std::path::PathBuf;

const SHOW_HIDDEN: bool = false;

#[derive(Clone)]
enum EntryType {
    File,
    Directory,
    BlockDevice,
    CharDevice,
    Fifo,
    Socket,
    SymlinkFile,
    SymlinkDir,
}

struct DirectoryEntery {
    name: String,
    entry_type: EntryType,
    is_hidden: bool,
}

impl DirectoryEntery {
    pub fn name(&self) -> &String {
        return &self.name;
    }

    pub fn entry_type(&self) -> &EntryType {
        return &self.entry_type;
    }

    pub fn is_hidden(&self) -> &bool {
        return &self.is_hidden;
    }

    pub fn new(name: &String, entry_type: &EntryType, is_hidden: &bool) -> Self {
        return Self {
            name: name.to_string().clone(),
            entry_type: entry_type.clone(),
            is_hidden: is_hidden.clone(),
        };
    }
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
                // TODO: Detect more types, move to function
                if file.is_file() {
                    res_buf.push(DirectoryEntery::new(
                        &filename.to_string(),
                        &EntryType::File,
                        &is_hidden(filename),
                    ));
                } else {
                    res_buf.push(DirectoryEntery::new(
                        &filename.to_string(),
                        &EntryType::Directory,
                        &is_hidden(filename),
                    ));
                }
            }
        }
    }

    return res_buf;
}

// ! Assuming name isn't empty
fn get_entry_name_without_dot(entry: &DirectoryEntery) -> String {
    let hidden = entry.is_hidden();
    let name = entry.name();
    if *hidden {
        let mut chars = name.chars();
        chars.next();
        chars.next_back();
        return chars.as_str().to_string();
    } else {
        return name.to_string();
    }
}

fn main() -> io::Result<()> {
    let entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut print_buf: String = String::new();

    let mut p_enteries: Vec<DirectoryEntery> = parse_pathbuffs(&entries);

    if SHOW_HIDDEN {
        p_enteries.push(DirectoryEntery::new(
            &String::from("."),
            &EntryType::Directory,
            &false,
        ));
        p_enteries.push(DirectoryEntery::new(
            &String::from(".."),
            &EntryType::Directory,
            &false,
        ));
    }

    p_enteries.sort_by_key(|a| get_entry_name_without_dot(&a));

    for entry in p_enteries {
        if !entry.is_hidden() || SHOW_HIDDEN {
            match entry.entry_type() {
                // TODO: Move formatting to function
                EntryType::Directory => {
                    if !entry.is_hidden() || SHOW_HIDDEN {
                        print_buf += &format!("\x1b[1;34m{}\x1b[0m  ", entry.name()).to_string();
                    }
                }
                _ => {
                    print_buf += &format!("{}  ", entry.name()).to_string();
                }
            }
        }
    }

    println!("{}", print_buf);

    Ok(())
}
