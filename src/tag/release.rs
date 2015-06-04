use std::fmt;
use std::collections::HashSet;

use git2::{Repository, Reference, ObjectType, Buf};

use package::Package;

pub struct Release<'a> {
    commit: &'a str,
    alias: Option<&'a str>,
    pkgs: HashSet<Package<'a>>,
    notes: &'a str,
}


impl<'a> fmt::Debug for Release<'a> {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Release alias: {:?}, pkgs: {:?}, notes: {:?}", self.alias, self.pkgs, self.notes)
        write!(f, "<{:?} @ {:?}>", self.pkgs, self.commit)
    }
}

impl<'a> Release<'a> {
    pub fn new (commit: &'a str, alias: Option<&'a str>, pkgs: HashSet<Package<'a>>, notes: &'a str) -> Release<'a> {
        Release {
            commit: commit,
            alias: alias,
            notes: notes,
            pkgs: pkgs,
        }
    }
    /*
    fn naive_tags () -> collections::HashSet<&'a str> {
    }
    */
}
