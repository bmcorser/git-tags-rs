use std::error;
use std::fs;
use std::fmt;
use std::io;
use std::path::Path;
use std::result::Result;

#[derive(Hash, Eq, PartialEq)]
pub struct Package<'a> {
    name: &'a Path,
}

impl<'a> fmt::Debug for Package<'a> {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

impl<'a> Package<'a> {
    pub fn new (path: &'a Path) -> Result<Package, io::Error> {
        try!(validate(&path));
        Ok(Package{name: &path})
    }
}

fn validate (path: &Path) -> Result<bool, io::Error> {
    try!(fs::metadata(path.join("deploy")));
    try!(fs::metadata(path.join("build")));
    Ok(true)
}
