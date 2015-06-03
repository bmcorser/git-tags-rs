use std::collections::HashSet;

use git2::{Repository, Reference};

use package::Package;

pub struct Release<'a> {
    commit: Reference<'a>,
    alias: Option<&'a str>,
    pkgs: HashSet<Package<'a>>,
}

impl<'a> Release<'a> {
    pub fn new (repo: &'a Repository, alias: Option<&'a str>, pkgs: HashSet<Package<'a>>) -> Release<'a> {
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
