use std::error::Error;
use std::fmt;
// use std::io;

#[derive(Debug)]
pub enum ReleaseError {
    TagExists,
}

impl fmt::Display for ReleaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReleaseError::TagExists => write!(f, "Tag exists"),
        }
    }
}

impl Error for ReleaseError {
    fn description(&self) -> &str {
        match *self {
            ReleaseError::TagExists => "A tag with that name already exists",
        }
    }
}

/*
impl From<io::Error> for ReleaseError {
    fn from(err: io::Error) -> ReleaseError {
        ReleaseError::Io(err)
    }
}
*/
