use std::fmt;
use std::error::Error;
use std::collections::HashSet;

use git2::{Repository, Reference, ObjectType, Buf};

use package;

pub struct Release<'a> {
    commit: &'a str,
    alias: Option<&'a str>,
    pkgs: HashSet<package::Package<'a>>,
    notes: &'a str,
}


impl<'a> fmt::Debug for Release<'a> {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Release alias: {:?}, pkgs: {:?}, notes: {:?}", self.alias, self.pkgs, self.notes)
        write!(f, "<{:?} @ {:?}>", self.pkgs, self.commit)
    }
}

impl<'a> Release<'a> {
    pub fn new (commit: &'a str, alias: Option<&'a str>, pkgs: HashSet<package::Package<'a>>, notes: &'a str)
        -> Release<'a> {
        Release {
            commit: commit,
            alias: alias,
            notes: notes,
            pkgs: pkgs,
        }
    }
}

#[test]
fn validate_pkgs_behaviour () {
    assert!(false);
}
