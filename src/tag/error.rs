use std::error::Error;
use std::fmt;
use std::io;
use git2;

#[derive(Debug)]
pub enum ReleaseError {
    Io,
    DirtyWorkTree,
    NoTrees,
    GitError,
}

impl fmt::Display for ReleaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReleaseError::DirtyWorkTree => write!(f, "Dirty working tree!"),
            ReleaseError::Io            => write!(f, "IO error!"),
            ReleaseError::NoTrees       => write!(f, "No trees to release!"),
            ReleaseError::GitError      => write!(f, "Git error!"),
        }
    }
}

impl Error for ReleaseError {
    fn description(&self) -> &str {
        match *self {
            ReleaseError::DirtyWorkTree         => "Dirty working tree!",
            ReleaseError::Io                    => "IO error!",
            ReleaseError::NoTrees               => "No trees to release!",
            ReleaseError::GitError              => "Git error!",
        }
    }
}

impl From<io::Error> for ReleaseError {
    fn from(_: io::Error) -> ReleaseError {
        ReleaseError::Io
    }
}

impl From<git2::Error> for ReleaseError {
    fn from(_: git2::Error) -> ReleaseError {
        ReleaseError::GitError
    }
}

#[derive(Debug)]
pub enum LookupError {
    GitError,
    NoChannel,
    NestingError,
    NotFound,
    EmptyChannel,
}

impl fmt::Display for LookupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LookupError::GitError     => write!(f, "Git error."),
            LookupError::NoChannel    => write!(f, "No channel supplied."),
            LookupError::NestingError => write!(f, "Nested packages disallowed."),
            LookupError::NotFound     => write!(f, "Release not found."),
            LookupError::EmptyChannel => write!(f, "Channel is empty."),
        }
    }
}

impl Error for LookupError {
    fn description(&self) -> &str {
        match *self {
            LookupError::GitError     => "Git error.",
            LookupError::NoChannel    => "No channel supplied.",
            LookupError::NestingError => "Nesting packages disallowed.",
            LookupError::NotFound     => "Release not found.",
            LookupError::EmptyChannel => "Channel is empty.",
        }
    }
}

impl From<git2::Error> for LookupError {
    fn from(_: git2::Error) -> LookupError {
        LookupError::GitError
    }
}
