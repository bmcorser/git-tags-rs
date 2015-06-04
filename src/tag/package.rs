use std::fmt;
use std::path::Path;
use std::fs;

#[derive(Hash, Eq, PartialEq)]
pub struct Package<'a> {
    name: &'a str,
}

impl<'a> fmt::Debug for Package<'a> {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}
impl<'a> Package<'a> {
    pub fn new (name: &'a str) -> Package {
        let deploy = fs::metadata(Path::new(&name).join("deploy"));
        let build = fs::metadata(Path::new(&name).join("build"));
        match (deploy, build) {
            (Result::Ok(_), Result::Ok(_))   => (),
            (Result::Err(_), Result::Ok(_))
                => panic!("I’m afraid {:?} is missing its deploy script.", name),
            (Result::Ok(_), Result::Err(_))
                => panic!("I’m afraid {:?} is missing its build script.", name),
            (Result::Err(_), Result::Err(_))
                => panic!("I’m afraid {:?} is missing deploy and build script.", name),
        }
        // check if it’s a package
        Package{name: name}
    }
}
