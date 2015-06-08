use std::error;
use std::fs;
use std::fmt;
use std::io;
use std::path::PathBuf;
use std::result::Result;

#[derive(Hash, Eq, PartialEq)]
pub struct ReleasePackage<'a> {
    pub name: &'a str,
    pub path: PathBuf,
}

impl<'a> fmt::Debug for Package<'a> {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

impl<'a> Package<'a> {
    pub fn new (path: PathBuf, name: &'a str) -> Result<Package<'a>, io::Error> {
        try!(validate(&path));
        Ok(Package{name: &name, path: path})
    }
}

