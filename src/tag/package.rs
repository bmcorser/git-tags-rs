#[derive(Hash, Debug, Eq, PartialEq)]
pub struct Package<'a> {
    name: &'a str,
}

impl<'a> Package<'a> {
    pub fn new (name: &'a str) -> Package {
        // check if it’s a package
        Package{name: name}
    }
}
