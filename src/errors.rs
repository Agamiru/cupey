use std::fmt;
use std::error::Error;
// use std::io;


#[derive(Debug)]
pub struct GenericError(pub String);

impl GenericError {
    pub fn new(description: &str) -> Self {
        return GenericError(description.to_owned());
    }

    pub fn to_boxed_err(&self) -> Box<Self> {
        Box::new(GenericError::new(&self.0))
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for GenericError {}


// impl From<std::io::Error> for GenericError {
//     fn from(err: std::io::Error) -> Self {
//         Self(err.to_string())
//     }
// }

// impl Into<io::Error> for GenericError {
//     fn into(self) -> io::Error {
//         io::Error {

//         }
//     }
// }


// struct CupeyError {
//     message: String,
//     error_kind:
// }





// enum ErrorKind {
//     io_error(std::io:Error)
// }