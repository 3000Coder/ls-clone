// Depends on unix APIs
#![cfg(target_family = "unix")]

use std::fs::{self};
use std::io;
use std::os::unix::fs::FileTypeExt;
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

fn get_file_type(file: &PathBuf) -> EntryType {
    let metadata = file.metadata().unwrap(); // TODO: Error handling
    let filetype = metadata.file_type();
    if metadata.is_symlink() {
        if metadata.is_dir() {
            return EntryType::SymlinkDir;
        } else if metadata.is_file() {
            return EntryType::SymlinkFile;
        } else {
            panic!("Invalid symlink type"); // ? Might be ok to remove later
        }
    } else if filetype.is_dir() {
        return EntryType::Directory;
    } else if filetype.is_file() {
        if filetype.is_block_device() {
            return EntryType::BlockDevice;
        } else if filetype.is_char_device() {
            return EntryType::CharDevice;
        } else if filetype.is_fifo() {
            return EntryType::Fifo;
        } else if filetype.is_socket() {
            return EntryType::Socket;
        } else {
            return EntryType::File;
        }
    } else {
        panic!("Invalid file type");
    }
}

fn parse_pathbuffs(files: &Vec<PathBuf>) -> Vec<DirectoryEntery> {
    let mut res_buf: Vec<DirectoryEntery> = Vec::new();

    for file in files {
        if let Some(f) = file.file_name() {
            if let Some(filename) = f.to_str() {
                res_buf.push(DirectoryEntery::new(
                    &filename.to_string(),
                    &get_file_type(&file),
                    &is_hidden(filename),
                ));
            }
        }
    }

    return res_buf;
}

fn get_ansi_code(entry_type: &EntryType) -> String {
    todo!()
}

fn get_entry_name_without_dot(entry: &DirectoryEntery) -> String {
    let hidden = entry.is_hidden().clone();
    let name = entry.name();

    assert!(name.len() > 0, "Name is empty!");
    if hidden {
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
