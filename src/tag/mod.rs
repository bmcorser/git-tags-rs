use std::collections::HashSet;

extern crate git2;
use git2::{Repository, Reference};

pub struct Package<'a> {
    name: &'a str,
}

impl<'a> Package<'a> {
    fn new (name: &'a str) -> Package {
        // check if it’s a package
        Package{name: name}
    }
}

pub struct Release<'a> {
    commit: Reference<'a>,
    alias: Option<&'a str>,
    pkgs: HashSet<Package<'a>>,
}

impl<'a> Release<'a> {
    fn new (
        repo: &'a Repository,
        alias: Option<&'a str>,
        pkgs: HashSet<Package<'a>>,
    ) -> Release<'a> {
        Release {
            commit: repo.head().unwrap(),
            alias: alias,
            pkgs: pkgs,
        }
    }
    /*
    fn naive_tags () -> collections::HashSet<&'a str> {
    }
    */
}
