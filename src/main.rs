// Depends on unix APIs
#![cfg(target_family = "unix")]

use std::fs::{self};
use std::io;
use std::os::unix::fs::FileTypeExt;
use std::path::PathBuf;
use termion::terminal_size;

const SHOW_HIDDEN: bool = true;

#[derive(Clone)]
enum EntryType {
    File,
    Directory,
    BlockDevice,
    CharDevice,
    Fifo,
    Socket,
    Symlink,
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
    let metadata = match file.symlink_metadata() {
        Ok(m) => m,
        Err(_) => return EntryType::File,
    };
    let filetype = metadata.file_type();
    if filetype.is_symlink() {
        return EntryType::Symlink;
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

// TODO: Account for file content
fn get_ansi_code(entry_type: &EntryType) -> &str {
    match entry_type {
        EntryType::File => return "",
        EntryType::Directory => return "\x1b[01;34m",
        EntryType::BlockDevice => "\x1b[01;33m",
        EntryType::CharDevice => "\x1b[01;33m",
        EntryType::Fifo => "\x1b[33m",
        EntryType::Socket => "\x1b[01;35m",
        EntryType::Symlink => "\x1b[01;36m",
    }
}

fn get_entry_name_without_dot(entry: &DirectoryEntery) -> String {
    let hidden = entry.is_hidden().clone();
    let name = entry.name();

    assert!(name.len() > 0, "Name is empty!");
    if hidden {
        let mut chars = name.chars();
        chars.next();
        return chars.as_str().to_string();
    } else {
        return name.to_string();
    }
}

fn calculate_rows(enteries: &Vec<DirectoryEntery>, term_width: u16, rows: usize) -> usize {
    assert!(rows != 0);
    if enteries.len() >= rows {
        return enteries.len();
    }

    let mut lens: Vec<usize> = vec![0; rows.try_into().unwrap()];

    for i in 0..enteries.len() {
        lens[i % rows as usize] += enteries[i].name().len() + 2;
        if lens[i % rows as usize] > term_width.into() {
            return calculate_rows(enteries, term_width, rows+1);
        }
    }
    return rows;
}

fn format_table(enteries: &Vec<DirectoryEntery>) -> String {
    let buf: String = String::new();
    let num_rows: usize = calculate_rows(&enteries, terminal_size().unwrap().0, 1);
    let colums: Vec<Vec<usize>> = vec![];
    for i in 0..num_rows {
        for j in 0..enteries.len().div_ceil(num_rows) {

        }
    }

    return buf;
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

    // TODO: Custom sorting key
    p_enteries.sort_by_key(|a| get_entry_name_without_dot(&a).to_lowercase());
    println!("{}", calculate_rows(&p_enteries, terminal_size().unwrap().0, 1));

    // TODO: Move to table
    for entry in p_enteries {
        if !entry.is_hidden() || SHOW_HIDDEN {
            if !entry.is_hidden() || SHOW_HIDDEN {
                print_buf += &format!(
                    "{}{}\x1b[0m  ",
                    get_ansi_code(entry.entry_type()),
                    entry.name()
                )
                .to_string();
            }
        }
    }

    println!("{}", print_buf);

    Ok(())
}
