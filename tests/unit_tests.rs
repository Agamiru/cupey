use Cupey;
use std::env;

mod common;


#[test]
fn visit_dirs_works() {
    let from_dir = common::cupey_test_folder_path();
        
    let mut to_dir = env::current_dir().unwrap();
    to_dir.push("to_dir");
    
    common::clean_up(&to_dir);      // remove existing test destination folders if exists before test
    Cupey::visit_dirs(from_dir.as_path(), &to_dir).unwrap();

    // Get folder sizes and save in variable so created folder can be 
    let from_dir_size = common::cupey_test_folder_size();
    let to_dir_size = common::folder_size(to_dir.as_path());
    let from_dir_count = common::cupey_test_folder_count();
    let to_dir_count = common::folder_count(to_dir.as_path());

    common::clean_up(&to_dir);      // remove test destination folders after creation.

    assert_eq!(from_dir_size, to_dir_size);
    assert_eq!(from_dir_count, to_dir_count)
}

#[test]
fn copier_works() {
    let file_name = "random_text_1.txt";
    let mut random_text_1 = common::cupey_test_folder_path();
    random_text_1.push(file_name);

    let mut dest_dir = env::current_dir().unwrap();
    Cupey::copier(&random_text_1, &mut dest_dir).unwrap();
    let file_created = dest_dir.exists();
    common::clean_up(dest_dir.as_path());       // Clean up created file
    assert!(file_created)
}



