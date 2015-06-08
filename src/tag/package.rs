use std::error;
use std::fs;
use std::fmt;
use std::io;
use std::path::Path;
use std::result::Result;

#[derive(Hash, Eq, PartialEq)]
pub struct Package<'a> {
    pub name: &'a str,
}

impl<'a> fmt::Debug for Package<'a> {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

impl<'a> Package<'a> {
    pub fn new (pkg_string: &'a str) -> Result<Package, io::Error> {
        let path = Path::new(pkg_string);
        try!(validate(&path));
        Ok(Package{name: &pkg_string})
    }
}

fn validate (path: &Path) -> Result<(), io::Error> {
    try!(fs::metadata(path.join("deploy")));
    try!(fs::metadata(path.join("build")));
    Ok(())
}
