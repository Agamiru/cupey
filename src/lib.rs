use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::fs;
use std::env;
use std::io::{Read, Write};

use clap::{Arg, App};

mod errors;
mod cupey_traits;

type GeneralResult = Result<(), errors::CupeyError>;


#[derive(Debug)]
pub struct Cupey<'a> {
    // origin: String,
    pub current_dir: PathBuf,
    pub matches: clap::ArgMatches<'a>,
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

        let overwrite_flag = Arg::with_name("overwrite")
            .help(
                    "Overrite all existing files the destination directory that share same name with file being copied."
                )
            .long("overwrite")
            .short("o");

        let app = app.arg(overwrite_flag);

        let matches = app.get_matches_from_safe(args)?;

        return Ok(Cupey {
            current_dir, matches
        });
    }
    
    pub fn copy_files(&self) -> GeneralResult {
        // from_arg has already been validated, safe to unwrap
        let originating_dir = self.matches.value_of("from_arg").unwrap();
        let overwrite = self.matches.is_present("overwrite");
        if overwrite {
            visit_dirs(&Path::new(originating_dir), &self.current_dir, true)?;    
        } else {
            visit_dirs(&Path::new(originating_dir), &self.current_dir, false)?;
        }

        Ok(())
    }
}


pub fn visit_dirs(dir: &Path, to_dir: &PathBuf, overwrite: bool) -> GeneralResult {

    if dir.is_file(){
        let message = format!("'{}' should not be a file", dir.to_str().unwrap());
        return Err(errors::CupeyError::new(message, errors::ErrorKind::DirIsFile))
    } else if to_dir.is_file() {
        let message = format!("'{}' should not be a file", to_dir.to_str().unwrap());
        return Err(errors::CupeyError::new(message, errors::ErrorKind::DirIsFile))
    }

    if empty_dir(dir) {
        let message = format!("This folder '{}' should not be empty", dir.to_str().unwrap());
        return Err(errors::CupeyError::new(message, errors::ErrorKind::DirEmpty))
    }

    // Recurse through folder.
    for entry in fs::read_dir(dir)? {

        let entry = entry?;
        let entry_path = entry.path();

        // block for handling folders
        if entry_path.is_dir() {

            // When entry is a folder
            if let Some(folder_name) = entry_path.file_name() {
                // Create a new directory in destination path
                let mut new_dest_dir = to_dir.to_owned();
                new_dest_dir.push(folder_name);
                
                if !new_dest_dir.exists() {
                    fs::create_dir_all(&new_dest_dir)?;
                }
                // Recurse through new directory
                visit_dirs(&entry_path, &new_dest_dir, overwrite)?;
            }
        
        // block for handling files     
        } else {
            // println!("Copying file {:?}", &entry_path);
            let mut new_dest_dir = to_dir.to_owned();
            copier(&entry_path.to_owned(), &mut new_dest_dir, overwrite)?;
        }
    }

    Ok(())
}

// orig_file_path - originating file path
pub fn copier(orig_file_path: &PathBuf, destination_dir: &mut PathBuf, overwrite: bool) -> GeneralResult {
    // Get file name to append to new destination path
    let file_name = orig_file_path.file_name().unwrap();
    destination_dir.push(file_name);
    // Change name for readability sakes.
    let destination_file_path = destination_dir;

    let mut dest_file;
    if destination_file_path.exists() {
        // Overwrite existing file 
        if overwrite {
            dest_file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(destination_file_path)?;
        } else {
            println!("Moving on, file exists: {:?}", &destination_file_path);
            return Ok(())
        }
    } else {
        // Create new file
        dest_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(destination_file_path)?;   
    }

    let mut file_to_copy = fs::OpenOptions::new()
        .read(true)
        .open(orig_file_path)?;
    
    let mut contents = Vec::new();  // Create Vec<u8> bytes buffer
    
    file_to_copy.read_to_end(&mut contents)?;

    dest_file.write_all(&mut contents)?;

    println!("Copied {:?} successfully", orig_file_path.as_path().file_name().unwrap());

    Ok(())
}

pub fn empty_dir(dir: &Path) -> bool {
    fs::read_dir(dir).into_iter().next().is_none()
}

