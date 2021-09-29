use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::fs;
use std::env;
use std::io::{BufRead, Read, Write, BufReader, Lines};

use clap::{Arg, SubCommand, App};

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
    origin: Option<String>,
    current_dir: PathBuf,
    matches: clap::ArgMatches<'a>,
    settings: Settings
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

        let setup_command = SubCommand::with_name("set-origin")
            .arg(
                Arg::with_name("originating_dir")
                    .help("The folder to be copied from")
                    .index(1)
                    .value_name("PATH")
                    .required(true)
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
                    })                
                );

        let app = app.subcommand(setup_command);

        let orig_dir_option = Arg::with_name("from-origin")
            .long("from-origin")
            .short("fo")
            .value_name("PATH")
            .help("Path to the folder you want to 'cupey' from");

        let matches = app.get_matches_from_safe(args)?;
        let settings = Settings::new();
        let origin_as_string: Option<String>;
        if settings.reader.is_some() {
            origin_as_string = Some(settings.origin_as_string()?);
        } else {
            origin_as_string = None;
        }

        return Ok(Cupey {
            origin: origin_as_string, current_dir, matches, settings
        });
        

    //     if let Some(arg_matches) = matches.subcommand_matches("set-origin") {
    //         let origin = arg_matches.value_of("originating_dir").unwrap().to_owned();
    //         let dest_file_path = Path::new(CUPEY_SETTINGS_FILE);
    //         write_from_string(&format!("{}\n", origin), dest_file_path).unwrap();

    //         return Ok(Cupey {origin, current_dir});
            
    //     } else if matches.is_present("from-origin") {
    //         let origin = ReadFromSettings::new()
    //             .origin_as_string()
    //             .map_err(|err| clap::Error {
    //                 message: format!("Couldn't get origin: {}", err.to_string()),
    //                 kind: clap::ErrorKind::InvalidValue, info: None
    //             })?;
    //             return Ok(Cupey {origin, current_dir});
            
    //     } else {
    //         return Err(clap::Error{
    //             message: "Improperly Configured".to_owned(),
    //             kind: clap::ErrorKind::ArgumentNotFound,
    //             info: None
    //         })
    //     }
    }


    fn set_origin(&mut self) {
        if let Some(arg_matches) = self.matches.subcommand_matches("set-origin") {
            let origin = arg_matches.value_of("originating_dir").unwrap().to_owned();
            let p = Settings::settings_file_path();
            let dest_file_path = Path::new(&p);
            self.settings.perform_write(
                dest_file_path, format!("{}\n", origin), Some("txt")
            );
            self.origin = Some(origin);
        }
    }

    
    fn copy_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = self.origin.clone().unwrap();
        visit_dirs(&Path::new(&path), &self.current_dir)?;
        Ok(())
    }
}


// impl<'a> cupey_traits::ResourceManager for Cupey<'a> {}


#[derive(Debug)]
pub struct Settings {
    reader: Option<LinesFromFile>
}

impl  Settings {

    fn new() -> Self {
        if Self::file_exists() {
            Self::from_file()
        } else {
            Self {reader: None}
        }
    }

    pub fn from_file() -> Self {
        let reader = Self::settings_file_lines()
            .map_err(|err| panic!(
                    "Failed to create Cupey object for this reason:\n'{}'",
                    err.to_string()
                )
            );
        Settings{ reader: Some(reader.unwrap()) }
    }

    pub fn settings_file_lines() -> Result<LinesFromFile, std::io::Error> {
        let settings_file = Self::settings_file()?;
        let settings_reader = BufReader::new(settings_file);
        Ok(settings_reader.lines())
    }

    // fn origin_file_path() -> &'static Path {
    //     return Path::new(Self::settings_file_path());
    // }

    fn settings_file() -> Result<fs::File, std::io::Error> {
        let settings_file = fs::OpenOptions::new()
            .read(true)
            .open(Path::new(&Self::settings_file_path()))?;
        
        Ok(settings_file)
    }

    fn origin_as_string(&self) -> Result<String, std::io::Error> {
        let mut reader = self.clone().reader
            .unwrap();
        Ok((reader.next().unwrap()?).to_owned())
    }

    fn settings_file_path() -> String {
        let path = format!("{}\\cupey_settings.txt", CUPEY_HOME_DIR);
        path
    }

    fn file_exists() -> bool {
        Path::new(&Self::settings_file_path()).exists()
    }

}

impl Clone for Settings {
    fn clone(&self) -> Self {
        Self::from_file()
    }
}

impl cupey_traits::ResourceManager for Settings {}



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


fn visit_dirs(dir: &Path, to_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {

    if dir.is_file() || to_dir.is_file() {
        println!("Neither of these should be files!");
        return Ok(());
    }

    if dir.read_dir()?.next().is_none() {
        println!("Folder is empty\n");
        return Ok(());
    }

    // Recurse through folder.
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            println!("Folder path: {:?}", path);
            if let Some(folder_name) = path.file_name() {
                // Create a new directory in destination path
                let mut new_dest_dir = to_dir.to_owned();
                new_dest_dir.push(folder_name);

                if !new_dest_dir.exists() {
                    fs::create_dir(&new_dest_dir)?;
                }
                visit_dirs(&path, &new_dest_dir)?;
            }
        
        // path is a file     
        } else {
            println!("Copying file {:?}", &path);

            let mut new_dest_dir = to_dir.to_owned();
            copier(&path, &mut new_dest_dir)?;
        }
    }

    Ok(())

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
    fn test_improperly_configured() {
        let cupey_inst = Cupey::new_from(["exename"].iter());
        assert!(cupey_inst.is_err());
        assert_eq!(&cupey_inst.unwrap_err().message, "Improperly Configured");
    }

    // #[test]
    fn test_properly_configured() {
        let cupey_inst = Cupey::new_from([
            "exename", "set-origin", "C:\\Users\\hp\\Desktop\\test_folder"
            ]
            .iter()).unwrap();
        assert!(cupey_inst.copy_files().is_ok());
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

    
    #[test]
    fn test_resource_manager() {
        let file_path = Path::new("C:\\Users\\hp\\Desktop\\Cupey");
        let test_struct = TestStruct{};
        let file_wrapper = test_struct.acquire(file_path, None).unwrap();
        assert_eq!(file_wrapper.exists, true);
    }


}
