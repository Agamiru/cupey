
use Cupey;
use std::env;
use std::fs;

mod common;

#[test]
fn cupey_works() {
    // get test folder path
    let cupey_test_folder_path = common::cupey_test_folder_path();

    let mut cupey_inst = Cupey::Cupey::new_from([
        "cupey", "--from", cupey_test_folder_path.to_str().unwrap(), "-o"
    ].iter()).unwrap();

    // Change cupey instance current dir (destination folder)
    // This is to avoid creating files in the current dir and messing up the place
    let mut to_dir = env::current_dir().unwrap();
    to_dir.push("to_dir");

    // Value will be moved eventually so create a copy
    let mut to_dir_copy = to_dir.to_owned();
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

    // Check overwrite
    // Re-create empty to_dir folder
    let new_to_dir = to_dir_copy.clone();
    fs::create_dir_all(to_dir_copy).unwrap();       // to_dir_copy moved here
    // Change the content of 'random_text_1.txt' in to_dir
    let mut random_text_1_path = new_to_dir.clone();
    random_text_1_path.push("random_text_1.txt");
    let content_string = "Shell camp";
    common::create_txt_file(content_string, &random_text_1_path, None);
    
    // Assert file create successful, assert content too
    assert!(random_text_1_path.exists());
    assert_eq!(common::read_to_string(&random_text_1_path), content_string.to_owned());
    
    // start application
    let mut cupey_inst = Cupey::Cupey::new_from([
        "cupey", "--from", cupey_test_folder_path.to_str().unwrap(), "--overwrite"
    ].iter()).unwrap();
    
    // Perform previous ritual
    cupey_inst.current_dir = new_to_dir.clone();

    // Cupey files
    cupey_inst.copy_files().unwrap();
    // get file from cupey test folder
    let mut cupey_random_text_1_path = common::cupey_test_folder_path();
    cupey_random_text_1_path.push("random_text_1.txt");
    let to_dir_random_text_1_path = common::read_to_string(&random_text_1_path);
    let cupey_random_text_1_string = common::read_to_string(&cupey_random_text_1_path);
    assert_eq!(cupey_random_text_1_string, to_dir_random_text_1_path);

    // Remove created folder
    common::clean_up(new_to_dir.as_path());

}


// #[test]
// fn cupey_overwrite() {

// }