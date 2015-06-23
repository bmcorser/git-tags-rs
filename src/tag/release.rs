use std::collections::{HashSet, HashMap};
use std::fmt;
use std::fs;
use std::io;
use std::env;
use std::hash::{Hash, Hasher};
use std::result::Result;
use std::path::{PathBuf, Path};
use std::error::Error;

use git2;

use error::ReleaseError;

static NAMESPACE: &'static str = "releases";

pub struct Release<'a> {
    repo: &'a git2::Repository,
    pub target: git2::Object<'a>,
    pub channel: &'a str,
    notes: Option<git2::Oid>,
    NAMESPACE: &'static str,
}

impl<'a> fmt::Debug for Release<'a> {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let short_id = self.target.short_id().unwrap();
        write!(f, "Release name {:?}", short_id.as_str().unwrap())
    }
}

fn validate_pkg (pkg_path: &PathBuf) -> Result<(), io::Error> {
    try!(fs::metadata(pkg_path.join("deploy")));
    try!(fs::metadata(pkg_path.join("build")));
    Ok(())
}


impl<'a> Release<'a> {

    pub fn new (repo: &'a git2::Repository,
                channel: &'a str,
                namespace: Option<&'static str>)
        -> Result<Release<'a>, ReleaseError> {

        let namespace = namespace.unwrap_or(NAMESPACE);
        let target = repo.revparse_single("HEAD").unwrap();

        /* todo: deep pkgs
        let cwd = env::current_dir().unwrap();
        let workdir = repo.workdir().unwrap();
        let repo_path = cwd.relative_from(workdir).unwrap();
        */

        let target_peeled = target.peel(git2::ObjectType::Tree).unwrap();
        let target_tree = target_peeled.as_tree().unwrap();

        let release = Release {
            repo: repo,
            target: target,
            channel: channel,
            notes: None,
            NAMESPACE: namespace,
        };
        Ok(release)
    }

    pub fn create_tag (&self) -> Result<(), ReleaseError> {
        let signature = self.repo.signature().unwrap();
        /*
        for (pkg_name, pkg_target) in self.pkgs.iter() {
            let tag_name = self.fmt_tag(&pkg_name);
            let tag_result = self.repo.tag(&tag_name, pkg_target, &signature, "", false);
            match tag_result {
                Err(_) => println!("Didn’t create tag: {:?}", tag_name),
                Ok(_) => println!("Created tag: {:?}", tag_name),
            };
        }
        */
        Ok(())
    }
}

#[test]
fn validate_pkgs_behaviour () {
    assert!(false);
}
