use std::error;
use std::fs;
use std::io;
use std::path::Path;
use std::result::Result;

#[derive(Hash, Debug, Eq, PartialEq)]
pub struct Package {
    name: Path,
}

impl Package {
    pub fn new (path: Path) -> Result<Package, io::Error> {
        try!(validate(&path));
        Ok(Package{name: path})
    }
}

fn validate (path: &Path) -> Result<(), io::Error> {
    try!(fs::metadata(path.join("deploy")));
            //.map_err(|e| Err("Missing deploy script")));
    try!(fs::metadata(path.join("build")));
            //.map_err(|e| Err("Missing build script")));
    Ok(())
}
