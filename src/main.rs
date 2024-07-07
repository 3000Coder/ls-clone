use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::path::PathBuf;

enum DirectoryEntery {
    HiddenDirectory(String),
    HiddenFile(String),
    Directory(String),
    File(String)
}

fn is_hidden(s: &str) -> bool {
    if s.len() > 0 {
        if s.chars().next().unwrap() == '.' {
            return true;
        }
    }
    return false;
}

fn pathbuff_to_names(files: Vec<PathBuf>) -> Vec<DirectoryEntery> {
    let mut res_buf: Vec<DirectoryEntery> = Vec::new();

    for file in files {
        if let Some(filename) = file.to_str() {
            match (is_hidden(filename), file.is_file()) {
                (false, false) => {
                    res_buf.push(DirectoryEntery::Directory(filename.to_string()));
                },
                (false, true) => {
                    res_buf.push(DirectoryEntery::File(filename.to_string()));
                },
                (true, false) => {
                    res_buf.push(DirectoryEntery::HiddenDirectory(filename.to_string()));
                },
                (true, true) => {
                    res_buf.push(DirectoryEntery::HiddenFile(filename.to_string()));
                }
            }
        }
    }

    return res_buf
}



fn main() {}