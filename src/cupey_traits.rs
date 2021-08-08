use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::fs;

use crate::{CUPEY_HOME_DIR, GenericError};


pub trait ResourceManager {

    fn acquire(&self, file_path: &Path, file_ext: Option<&str>) -> Result<FileWrapper, Box<dyn std::error::Error>> {
        if let Some(ext) = file_ext {
            if let Some(file_extention) = get_extension_from_filename(&file_path.to_str().unwrap()) {
                if file_extention != ext {
                    return Err(GenericError::new("Wrong file type, must be a text file")
                        .to_boxed_err());
                }
            } else {
                return Err(GenericError::new("Wrong file type, must be a text file")
                    .to_boxed_err());
            } 
        }

        let acquired_file: FileWrapper;

        if !file_path.exists() {
            acquired_file = FileWrapper::new(file_path, false);

        } else {
            acquired_file = FileWrapper::new(file_path, true);
        }
        
        Ok(acquired_file)
    }

    // Sets temp_file_path to FileWrapper
    fn duplicate(&self, mut acquired: FileWrapper) -> Result<FileWrapper, Box<dyn std::error::Error>> {
        if acquired.exists {
            // Make a duplicate of file to a temporary path
            copier(&acquired.file_path_to_pathbuf(), &mut acquired.temp_file_dir())?;
            fs::remove_file(&acquired.file_path)?;
        }
        Ok(acquired)
    }

    fn write_and_drop(&self, string: &str, dest_file_path: &Path, acquired: FileWrapper) -> Result<(), Box<dyn std::error::Error>> {
        let mut dest_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(dest_file_path)?;

        dest_file.write(string.as_bytes())?;

        if acquired.exists {
            fs::remove_file(acquired.temp_file_path.unwrap())?;
        }
        Ok(())
    }

    fn perform_write(&self, file_path: &Path, string: String, file_ext: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let acquired = self.acquire(file_path, file_ext)?;
        self.write_and_drop(
            &string, 
            file_path,
            self.duplicate(acquired)?
        )?;
        Ok(())
    }

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



pub struct FileWrapper {
    pub file_path: String,
    pub exists: bool,
    pub temp_file_path: Option<PathBuf>
}

impl FileWrapper {
    pub fn new(file_path: &Path, exists: bool) -> Self {
        let string_path = file_path.to_str().unwrap().to_owned();
        Self {file_path: string_path, exists, temp_file_path: None}
    }

    fn file_path_to_pathbuf(&self) -> PathBuf {
        Path::new(&self.file_path).to_owned()
    }

    fn temp_file_dir(&mut self) -> PathBuf{
        let path = Path::new(&self.file_path);
        let mut temp_file_dir = PathBuf::from(CUPEY_HOME_DIR);
        temp_file_dir.push("tmp");
        let mut temp_file_path = temp_file_dir.clone();
        temp_file_path.push(path.file_name().unwrap());
        self.temp_file_path = Some(temp_file_path);
        temp_file_dir      
    } 
}