use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ReleaseError {
    TagExists,
    Io,
    PackagePathDisallowed,
}

impl fmt::Display for ReleaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReleaseError::TagExists             => write!(f, "Tag exists"),
            ReleaseError::Io                    => write!(f, "I forget."),
            ReleaseError::PackagePathDisallowed => write!(f, "Not allowed to use ../ in package spec."),
        }
    }
}

impl Error for ReleaseError {
    fn description(&self) -> &str {
        match *self {
            ReleaseError::TagExists             => "A tag with that name already exists",
            ReleaseError::PackagePathDisallowed => "Not allowed to use ../ in package spec.",
            // ReleaseError::AlreadyReleased => "Package already released",
            ReleaseError::Io                    => "I forget",
        }
    }
}

impl From<io::Error> for ReleaseError {
    fn from(err: io::Error) -> ReleaseError {
        ReleaseError::Io
    }
}
