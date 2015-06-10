use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::env;
use std::hash::{Hash, Hasher};
use std::result::Result;
use std::path::PathBuf;
use std::error::Error;

use git2;

use error::ReleaseError;

static NAMESPACE: &'static str = "releases";

pub struct Release<'a> {
    repo: &'a git2::Repository,
    target: git2::Object<'a>,
    abbrev_commit: String,
    pkgs: HashSet<String>,
    notes: &'a str,
    NAMESPACE: &'static str,
}

impl<'a> fmt::Debug for Release<'a> {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Packages: {:?} Commit: {:?} Tags: {:?}>", self.pkgs, self.abbrev_commit, self.tag_names())
    }
}

fn validate_pkgs (repo: &git2::Repository, pkgs: &HashSet<String>) -> Result<(), io::Error> {
    for pkg_name in pkgs {
        let pkg_path = repo.workdir().unwrap().join(&pkg_name);
        match fs::metadata(&pkg_path) {
            Ok(_)    => (),
            Err(err) => {
                println!("{:?} doesn’t exist.", pkg_name);
            }
        }
        match validate_pkg(&pkg_path) {
            Ok(_)    => (),
            Err(err) => {
                println!("{:?} is not a valid package.", pkg_name);
            }
        }
    }
    Ok(())
}

fn validate_pkg (pkg_path: &PathBuf) -> Result<(), io::Error> {
    try!(fs::metadata(pkg_path.join("deploy")));
    try!(fs::metadata(pkg_path.join("build")));
    Ok(())
}

impl<'a> Release<'a> {

    pub fn new (repo: &'a git2::Repository,
                commit: &'a str,
                pkgs: HashSet<String>,
                notes: &'a str,
                namespace: Option<&'static str>)
        -> Result<Release<'a>, ReleaseError> {
        let namespace = namespace.unwrap_or(NAMESPACE);
        let target = repo.revparse_single(commit).unwrap();
        let abbrev_commit = String::from_utf8(target.short_id().unwrap().to_vec()).unwrap();
        try!(validate_pkgs(&repo, &pkgs));
        Ok(Release {
            repo: repo,
            target: target,
            abbrev_commit: abbrev_commit,
            notes: notes,
            pkgs: pkgs,
            NAMESPACE: namespace,
        })
    }

    fn fmt_tag(&self, pkg: &str, commit: &str) -> String {
        format!("{namespace}/{pkg}/{commit}",
                namespace=self.NAMESPACE,
                pkg=pkg,
                commit=commit)
    }

    fn existing_tags (&self) -> git2::References {
        self.repo.references_glob(self.NAMESPACE).unwrap()
    }

    pub fn tag_names (&self) -> HashSet<String> {
        let mut tags = HashSet::new();
        for pkg in &self.pkgs {
            tags.insert(self.fmt_tag(&pkg, &self.abbrev_commit));
        }
        tags
    }

    pub fn validate_unreleased (&self) -> Result<(), ReleaseError> {
        let glob = format!("refs/tags/{}/*", self.NAMESPACE);
        for reference in self.repo.references_glob(&glob).unwrap() {
            println!("{:?}", reference.name());
            let mut revwalk = self.repo.revwalk().unwrap();
            revwalk.set_sorting(git2::SORT_TOPOLOGICAL);
            println!("  {:?}", revwalk);
            /*
            for rev in revwalk.push_ref(&self.abbrev_commit).unwrap() {
                println!("  {:?}", rev);
            }
            */
        }
        Ok(())
    }

    // pub fn validate_tags (&self) -> Result<HashSet<&str>, ReleaseError> {
    pub fn validate_tags (&self) -> Result<(), ReleaseError> {
        for tag_name in &self.tag_names() {
            match self.repo.revparse_single(tag_name) {
                Ok(_)  => return Err(ReleaseError::TagExists),
                Err(_) => Ok::<(), ReleaseError>(()),
            };
        };
        Ok(())
    }


    /*
    fn new_tags (&self) -> HashSet<&str> {
    }
    pub fn new_tags (&self) -> HashSet<&str> {
        let mut tags = HashSet::new();
    }
    */
}

#[test]
fn validate_pkgs_behaviour () {
    assert!(false);
}
