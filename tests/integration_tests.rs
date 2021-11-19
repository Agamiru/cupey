
use Cupey;
use std::env;

mod common;

#[test]
fn cupey_works() {
    // get test folder path
    let cupey_test_folder_path = common::cupey_test_folder_path();

    let mut cupey_inst = Cupey::Cupey::new_from([
        "cupey", "--from", cupey_test_folder_path.to_str().unwrap()
    ].iter()).unwrap();

    // Change cupey instance current dir (destination folder)
    // This is to avoid creating files in the current dir and messing up the place
    let mut to_dir = env::current_dir().unwrap();
    to_dir.push("to_dir");

    // Value will be moved eventually so create a copy
    let to_dir_copy = to_dir.to_owned();
    cupey_inst.current_dir = to_dir;    // to_dir moved here
    
    // Just incase this dir already exists, remove it.
    common::clean_up(to_dir_copy.as_path());
    
    // Cupey files
    cupey_inst.copy_files().unwrap();

    // Get
    let from_dir_size = common::cupey_test_folder_size();
    let to_dir_size = common::folder_size(to_dir_copy.as_path());
    let from_dir_count = common::cupey_test_folder_count();
    let to_dir_count = common::folder_count(to_dir_copy.as_path());

    // Remove created folder
    common::clean_up(to_dir_copy.as_path());

    // Assert correctness
    assert_eq!(from_dir_size, to_dir_size);
    assert_eq!(from_dir_count, to_dir_count);

}