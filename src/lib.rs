use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::fs;
use std::env;
use std::io::{BufRead, Read, Write, BufReader, Lines};

use clap::{Arg, SubCommand, App};
use walkdir::WalkDir;

mod errors;
mod cupey_traits;
mod settings_file;

use crate::cupey_traits::ResourceManager;
use crate::errors::GenericError;

const CUPEY_HOME_DIR: &str = "C:\\Users\\hp\\Desktop\\Cupey";
// cupey_settings_file = "C:\\Users\\hp\\Desktop\\Cupey\\cupey_settings.txt";

type LinesFromFile = Lines<BufReader<fs::File>>;

#[derive(Debug)]
pub struct Cupey<'a> {
    // origin: String,
    current_dir: PathBuf,
    matches: clap::ArgMatches<'a>,
    // roll_back: Option<String>,
}

impl<'a> Cupey<'a> {
    pub fn new() -> Self {
        Self::new_from(env::args_os().into_iter()).unwrap_or_else(|e| e.exit())
    }

    pub fn new_from<I, T>(args: I) -> Result<Self, clap::Error> 
    where 
        I: Iterator<Item = T>, 
        T: Into<OsString> + Clone,
    {
        let current_dir = env::current_dir()?;

        let app = App::new("cupey")
            .version("0.0.1")
            .about("Recursively copy files from one folder to another")
            .author("Chidi Nnadi");

        let from_arg = Arg::with_name("from_arg")
            .help("The folder location to copy from")
            .short("f")
            .long("from")
            .value_name("PATH")
            .takes_value(true)
            .validator(|value| {
                let path = Path::new(&value);
                if path.exists() {
                    if path.is_dir() {
                        return Ok(());
                    }
                    return Err("Sorry this path isn't a directory".to_owned());
                } else {
                    return Err("Sorry this path doesnt't exist".to_owned());
                }
            });

        let app = app.arg(from_arg);

        let matches = app.get_matches_from_safe(args)?;

        return Ok(Cupey {
            current_dir, matches
        });
    }
    
    pub fn copy_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        let originating_dir = self.matches.value_of("from_arg").unwrap();
        visit_dirs(&Path::new(originating_dir), &self.current_dir)?;
        Ok(())
    }
}

fn visit_dirs(dir: &Path, to_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {

    // let mut perms = fs::metadata(dir)?.permissions();
    // perms.set_readonly(true);
    // fs::set_permissions(dir, perms)?;
    // println!("I ran");

    if dir.is_file() || to_dir.is_file() {
        println!("Neither of these should be files!");
        return Ok(());
    }

    // if fs::read_dir(dir)?.next().is_none() {
    //     println!("Folder is empty\n");
    //     return Ok(());
    // }
    // Check if folder is empty
    if fs::read_dir(dir).into_iter().next().is_none() {
    // if WalkDir::new(dir).into_iter().next().is_none() {
        println!("Folder is empty\n");
        return Ok(());
    }

    println!("I ran");

    // Recurse through folder.
    for entry in fs::read_dir(dir)? {
    // for entry in WalkDir::new(dir) {
        let entry = entry?;
        let entry_path = entry.path();

        println!("entry_path: {:?}", entry_path );

        // block for handling folders
        if entry_path.is_dir() {
            println!("Path is folder");
            println!("Folder path: {:?}", entry_path);
            if let Some(folder_name) = entry_path.file_name() {
                // Create a new directory in destination path
                let mut new_dest_dir = to_dir.to_owned();
                new_dest_dir.push(folder_name);

                if !new_dest_dir.exists() {
                    fs::create_dir(&new_dest_dir)?;
                }
                visit_dirs(&entry_path, &new_dest_dir)?;
            }
        
        // block for handling files     
        } else {
            println!("Copying file {:?}", &entry_path);

            let mut new_dest_dir = to_dir.to_owned();
            copier(&entry_path.to_owned(), &mut new_dest_dir)?;
        }
    }

    Ok(())

}

fn write_from_string(string: &str, dest_file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Check if file is a text file
    if let Some(file_extention) = get_extension_from_filename(&dest_file_path.to_str().unwrap()) {
        if file_extention != "txt" {
            return Err(GenericError::new("Wrong file type, must be a text file")
                .to_boxed_err());
        }
    } else {
        return Err(GenericError::new("Wrong file type, must be a text file")
            .to_boxed_err());
    }
    
    let mut dest_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(dest_file_path)?;

    dest_file.write(string.as_bytes())?;
    
    Ok(())
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
}


fn copier(orig_file_path: &PathBuf, destination_dir: &mut PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Get file name to append to new destination path
    let file_name = orig_file_path.file_name().unwrap();
    destination_dir.push(file_name);
    
    if destination_dir.exists() {
        println!("Moving on, file exists: {:?}", &destination_dir);
        return Ok(())
    }

    let mut file_to_copy = fs::OpenOptions::new()
        .read(true)
        .open(orig_file_path)?;
    
    let mut contents = Vec::new();  // Create Vec<u8> bytes buffer

    let mut dest_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(destination_dir)?;

        
    file_to_copy.read_to_end(&mut contents)?;

    dest_file.write_all(&mut contents)?;

    println!("File copied: {:?}\n", &file_to_copy);

    Ok(())
}
    




#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_improperly_configured() {
    //     let cupey_inst = Cupey::new_from(["exename"].iter());
    //     assert!(cupey_inst.is_err());
    //     assert_eq!(&cupey_inst.unwrap_err().message, "Improperly Configured");
    // }

    // #[test]
    fn test_properly_configured() {
        let mut cupey_inst = Cupey::new_from([
            "cupey", "--from", "/Users/mac/Desktop/cupey_from"
            ]
            .iter()).unwrap();
        let mut current_dir = PathBuf::new();
        current_dir.push(Path::new("/Users/mac/Desktop/cupey_into"));
        cupey_inst.current_dir = current_dir;
        let err  = cupey_inst.copy_files().unwrap();
        println!("{:?}", err);
        // assert!(cupey_inst.copy_files().is_ok());
    }

    #[test]
    fn test_walkdir() {
        let copy_from = Path::new("/Users/mac/Desktop/cupey_from");
        // for dir_entry in WalkDir::new(copy_from) {
        for dir_entry in copy_from.read_dir().unwrap() {
            println!(" I ran");
            // let entry = dir_entry.unwrap();
            println!("{:?}", dir_entry)
        }
        // println!("{:?}", fs::read_dir(copy_from).unwrap())

    }

    // #[test]
    fn test_write_file() {
        let dest_file_path = Path::new("C:\\Users\\hp\\Desktop\\save.txt");
        let string_to_read = "hi niggaz".to_owned();
        write_from_string(&string_to_read, dest_file_path).unwrap();

        // Closure to catch assert panic so created file can be deleted no matter
        // the outcome.
        let error_closure = || -> Result<(), std::io::Error> {
            let mut dest_file = fs::OpenOptions::new()
            .read(true)
            .open(dest_file_path)?; 
        
            let mut read_string = String::new();
            dest_file.read_to_string(&mut read_string)?;

            assert_eq!(read_string, string_to_read);    // Could panic

            Ok(())
        };

        let result = std::panic::catch_unwind(|| {
                error_closure().unwrap()
            });
        
        // Remove created file if exists
        if dest_file_path.exists() {
            fs::remove_file(dest_file_path).unwrap();
        }

        // The only allowed panic
        assert!(result.is_ok());
        
    }

    struct TestStruct;

    impl ResourceManager for TestStruct {}

    
    // #[test]
    fn test_resource_manager() {
        let file_path = Path::new("C:\\Users\\hp\\Desktop\\Cupey");
        let test_struct = TestStruct{};
        let file_wrapper = test_struct.acquire(file_path, None).unwrap();
        assert_eq!(file_wrapper.exists, true);
    }


}
