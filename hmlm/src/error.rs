use std::fmt;
use std::result;
use std::io;
use std::error::Error;
/// Error for all HMLM
#[derive(Debug)]
pub enum HmlmError {
    /// An I/O error occured in the underlying 'Read' or 'Write'
    Io(io::Error),

}

pub type HmlmResult<T> = result::Result<T, HmlmError>;

impl From<io::Error> for HmlmError {
    fn from(err: io::Error) -> HmlmError {
        HmlmError::Io(err)
    }
}

impl fmt::Display for HmlmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "Hmlm error: ")?;
        match *self {
            HmlmError::Io(ref e) =>
                write!(f, "I/O error: {}", e),
        }
    }
}

impl Error for HmlmError {
    fn description(&self) -> &str {
        match *self {
            HmlmError::Io(_) =>
                "I/O error",
        }
    }
}

