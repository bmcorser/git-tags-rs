use std::error::Error;
use std::fmt;
use std::io;
use git2;

#[derive(Debug)]
pub enum ReleaseError {
    Io,
    DirtyWorkTree,
    PackagePathDisallowed,
    AlreadyReleased,
    NoTrees,
    GitError,
}

impl fmt::Display for ReleaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReleaseError::DirtyWorkTree         => write!(f, "Dirty working tree."),
            ReleaseError::AlreadyReleased       => write!(f, "Already released"),
            ReleaseError::Io                    => write!(f, "I forget."),
            ReleaseError::PackagePathDisallowed => write!(f, "Not allowed to use ../ in package spec."),
            ReleaseError::NoTrees               => write!(f, "No trees."),
            ReleaseError::GitError              => write!(f, "Git error."),
        }
    }
}

impl Error for ReleaseError {
    fn description(&self) -> &str {
        match *self {
            ReleaseError::DirtyWorkTree         => "Dirty working tree.",
            ReleaseError::AlreadyReleased       => "Already released",
            ReleaseError::PackagePathDisallowed => "Not allowed to use ../ in package spec.",
            ReleaseError::Io                    => "I forget",
            ReleaseError::NoTrees               => "No trees.",
            ReleaseError::GitError              => "Git error.",
        }
    }
}

impl From<io::Error> for ReleaseError {
    fn from(err: io::Error) -> ReleaseError {
        ReleaseError::Io
    }
}

impl From<git2::Error> for ReleaseError {
    fn from(err: git2::Error) -> ReleaseError {
        ReleaseError::GitError
    }
}

#[derive(Debug)]
pub enum LookupError {
    GitError,
}

impl fmt::Display for LookupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LookupError::GitError => write!(f, "Git error."),
        }
    }
}

impl Error for LookupError {
    fn description(&self) -> &str {
        match *self {
            LookupError::GitError => "Git error.",
        }
    }
}

impl From<git2::Error> for LookupError {
    fn from(err: git2::Error) -> LookupError {
        LookupError::GitError
    }
}
