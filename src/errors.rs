use std::fmt;
use std::process;
use std::io::Write;

use clap;


#[derive(Debug)]
pub enum ErrorKind {
    OsError,
    ClapError,
    DirIsFile,
    DirEmpty,
    IoError
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            ErrorKind::ClapError => write!(f, "Clap Error"),
            ErrorKind::IoError => write!(f, "IO Error"),
            ErrorKind::DirEmpty => write!(f, "DirEmpty"),
            ErrorKind::DirIsFile => write!(f, "DirIsFile"),
            ErrorKind::OsError => write!(f, "OS Error")
        }
    }
}



#[derive(Debug)]
pub struct CupeyError {
    message: String,
    error_kind: ErrorKind
}

impl CupeyError {
    pub fn new(message: String, error_kind: ErrorKind) -> Self {
        CupeyError { message: message, error_kind: error_kind}
    }

    pub fn exit(&self) {
        // Write to stdout before exiting
        let out = std::io::stdout();
        writeln!(&mut out.lock(), "{}", self.to_string()).expect("Failed to write to stdout");
        process::exit(0)
    }
}

// Impelementations
impl fmt::Display for CupeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Error Type: {}\nError Message: {}",
            self.message, self.error_kind.to_string()
        )
    }
}

impl std::error::Error for CupeyError {}

// Conversions
impl From<std::io::Error> for CupeyError {
    fn from(err: std::io::Error) -> Self {
        CupeyError {
            message: err.to_string(),
            error_kind: ErrorKind::OsError
        }
    }
}

impl From<clap::Error> for CupeyError {
    fn from(err: clap::Error) -> Self {
        CupeyError {
            message: err.to_string(),
            error_kind: ErrorKind::ClapError
        }
    }
}
