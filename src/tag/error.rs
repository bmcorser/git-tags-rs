use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ReleaseError {
    Io,
    PackagePathDisallowed,
    AlreadyReleased,
    NoTrees,
}

impl fmt::Display for ReleaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReleaseError::AlreadyReleased       => write!(f, "Already released"),
            ReleaseError::Io                    => write!(f, "I forget."),
            ReleaseError::PackagePathDisallowed => write!(f, "Not allowed to use ../ in package spec."),
            ReleaseError::NoTrees               => write!(f, "No trees."),
        }
    }
}

impl Error for ReleaseError {
    fn description(&self) -> &str {
        match *self {
            ReleaseError::AlreadyReleased       => "Already released",
            ReleaseError::PackagePathDisallowed => "Not allowed to use ../ in package spec.",
            // ReleaseError::AlreadyReleased => "Package already released",
            ReleaseError::Io                    => "I forget",
            ReleaseError::NoTrees               => "No trees.",
        }
    }
}

impl From<io::Error> for ReleaseError {
    fn from(err: io::Error) -> ReleaseError {
        ReleaseError::Io
    }
}
