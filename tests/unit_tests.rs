use Cupey;
use std::env;
use std::fs;
use std::path;


mod common;

type GeneralErrors = Result<(), Box<dyn std::error::Error>>;


#[test]
fn visit_dirs_works() {
    let from_dir = common::cupey_test_folder_path();
        
    let mut to_dir = env::current_dir().unwrap();
    to_dir.push("to_dir");
    
    common::clean_up(&to_dir);      // remove existing test destination folders if exists before test
    Cupey::visit_dirs(from_dir.as_path(), &to_dir, true).unwrap();

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
    Cupey::copier(&random_text_1, &mut dest_dir, false).unwrap();
    let file_created = dest_dir.exists();
    common::clean_up(dest_dir.as_path());       // Clean up created file
    assert!(file_created)
}

#[test]
fn copier_overwrite_works() {
    // create text_file path in current_dir
    let mut cupey_test_folder = common::cupey_test_folder_path();
    let mut orig_file_path = cupey_test_folder.clone();
    orig_file_path.push("some_text.txt");

    // create new text file in cupey_text_folder and write text into it.
    common::create_txt_file("camp", &orig_file_path, None);

    // dest_file_dir
    let current_dir = env::current_dir().unwrap();
    let mut dest_file_path = current_dir.clone();
    dest_file_path.push("some_text.txt");

    // create previous text file in current_dir and write different text into it.
    common::create_txt_file("shell", &dest_file_path, None);

    // copy newly created file to cupey test folder, which has a similar file with different content
    Cupey::copier(&mut dest_file_path, &mut cupey_test_folder, true).unwrap();

    // assert copied file is same with originating file
    assert_eq!(String::from("shell"), common::read_to_string(&mut dest_file_path));

    common::clean_up(orig_file_path.as_path());
    common::clean_up(dest_file_path.as_path());
}

// fn copier_skip_works() {
//     // create text_file path in current_dir
//     let mut orig_file_path = env::current_dir().unwrap();
//     orig_file_path.push("some_text.txt");

//     // create text file in current_dir and write text into it.
//     common::create_txt_file("shell", &mut orig_file_path, None);
    
//     // dest_file_dir
//     let mut dest_file_dir = common::cupey_test_folder_path();
//     assert!(dest_file_dir.exists());

//     // copy newly created file to new directory
//     Cupey::copier(&mut orig_file_path, &mut dest_file_dir, false).unwrap();

//     // check if copy was successful
//     let mut dest_file_path = dest_file_dir;
//     println!("{:?}", dest_file_path.as_path());
//     assert!(dest_file_path.exists());

//     // overrite content of copied file in new directory
//     common::create_txt_file("camp", &mut dest_file_path, Some(true));
    

// }



