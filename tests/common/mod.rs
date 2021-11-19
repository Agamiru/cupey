use std::path;
use std::fs;
use std::env;

use Cupey;




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