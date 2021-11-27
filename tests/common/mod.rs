use std::path;
use std::fs;
use std::env;
use std::io::{Read, Write};

use Cupey;

/// Test utils functions will Panic on errors, they are intended to be simple
/// and not propagate errors.


pub const TEST_FOLDER_NAME: &str = "cupey_test_folder";


pub fn clean_up(dir: &path::Path) {
    if dir.exists() {
        match dir.is_dir() {
            true => fs::remove_dir_all(dir).unwrap(),
            false => fs::remove_file(dir).unwrap()
        }
    }
}

pub fn cupey_test_folder_path() -> std::path::PathBuf {
    let mut cupey_test_folder_path = env::current_dir().unwrap();
    cupey_test_folder_path.push(TEST_FOLDER_NAME);
    cupey_test_folder_path
}

pub fn folder_size(dir: &path::Path) -> u64 {

    fn recurse(vec_: &mut Vec<u64>, dir: &path::Path) -> u64 {
        for entry in fs::read_dir(dir).unwrap() {
            if let Ok(dir_entry) = entry {
                if dir_entry.path().is_dir() {
                    recurse(vec_, &dir_entry.path());
                } else {
                    let entry_size = dir_entry.metadata().unwrap().len();
                    vec_.push(entry_size);
                }
            }
        }
        vec_.iter().sum()
    }

    match Cupey::empty_dir(dir) {
        true => 0,
        false => {
            let mut size_vec: Vec<u64> = Vec::new();
            recurse(&mut size_vec, dir)
        }
    }
}

// Todo: Implement recursion counting for this function.
// For now it just counts the files (folders included) in the given dir.
pub fn folder_count(dir: &path::Path) -> u64 {
    match Cupey::empty_dir(dir) {
        true => 0,
        false => {
            let mut count = 0;
            for entry in fs::read_dir(dir).unwrap() {
                if let Ok(_) = entry {
                    count += 1;
                }
            }
            count
        }
    }
}

pub fn cupey_test_folder_size() -> u64 {
    folder_size(cupey_test_folder_path().as_path())
}

pub fn cupey_test_folder_count() -> u64 {
    folder_count(cupey_test_folder_path().as_path())
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    path::Path::new(filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
}

pub fn create_txt_file(text: &str, dest_file_path: &path::Path, overwrite: Option<bool>) {

    // Check if file is a text file
    if let Some(file_extention) = get_extension_from_filename(&dest_file_path.to_str().unwrap()) {
        if file_extention == "txt" {
            write_from_string(&text, dest_file_path, overwrite);
        }
    }
}


fn write_from_string(string: &str, dest_file_path: &path::Path, overwrite: Option<bool>) {
    let mut dest_file;
    
    match overwrite {
        // Create new file, fail if exists
        None => {
            dest_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(dest_file_path)
                .unwrap();
        },
        // Create new file, overwrite if exists
        Some(true) => {
            dest_file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(dest_file_path)
                .unwrap();
        },
        // Create new file, append if exists
        Some(false) => {
            dest_file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(dest_file_path)
                .unwrap();
        }
    }

    dest_file.write(string.as_bytes()).unwrap();
    
}

pub fn read_to_string(file_path: &path::Path) -> String {
    let mut string_buffer = String::new();

    let mut file_to_read = fs::OpenOptions::new()
        .read(true)
        .open(file_path)
        .unwrap();

    file_to_read.read_to_string(&mut string_buffer).unwrap();

    string_buffer
}