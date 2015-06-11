use std::collections::{HashSet, HashMap};
use std::fmt;
use std::fs;
use std::io;
use std::env;
use std::hash::{Hash, Hasher};
use std::result::Result;
use std::path::{PathBuf,Path};
use std::error::Error;

use git2;

use error::ReleaseError;

static NAMESPACE: &'static str = "releases";

pub struct Release<'a> {
    repo: &'a git2::Repository,
    target: git2::Object<'a>,
    pkgs: HashMap<&'a str, git2::Object<'a>>,
    notes: &'a str,
    NAMESPACE: &'static str,
}

impl<'a> fmt::Debug for Release<'a> {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let short_id = self.target.short_id().unwrap();
        write!(f, "Release at {:?} of: \n", short_id.as_str().unwrap());
        for (name, object) in self.pkgs.iter() {
            write!(f, "  {:?} -> {:?}\n", name, object.id());
        };
        Ok(())
    }
}

fn validate_pkgs (repo: &git2::Repository, pkgs: &HashMap<&str, git2::Tree>) -> Result<(), ReleaseError> {
    for (pkg_name, _) in pkgs.iter() {
        let pkg_path = repo.workdir().unwrap().join(&pkg_name);
        match fs::metadata(&pkg_path) {
            Ok(_)    => (),
            Err(err) => {
                println!("{:?} doesn’t exist.", pkg_name);
                return Err(ReleaseError::Io);
            }
        }
        match validate_pkg(&pkg_path) {
            Ok(_)    => (),
            Err(err) => {
                println!("{:?} is not a valid package.", pkg_name);
                return Err(ReleaseError::Io);
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
                pkg_specs: Vec<&'a str>,
                notes: &'a str,
                namespace: Option<&'static str>)
        -> Result<Release<'a>, ReleaseError> {
        // -> Result<(), ReleaseError> {

        let namespace = namespace.unwrap_or(NAMESPACE);

        let target = repo.revparse_single(commit).unwrap();

        /* todo: deep pkgs
        let cwd = env::current_dir().unwrap();
        let workdir = repo.workdir().unwrap();
        let repo_path = cwd.relative_from(workdir).unwrap();
        */

        let target_peeled = target.peel(git2::ObjectType::Tree).unwrap();
        let target_tree = target_peeled.as_tree().unwrap();

        let mut pkgs = HashMap::new();

        for pkg_spec in pkg_specs {
            if pkg_spec.contains("/") {
                return Err(ReleaseError::PackagePathDisallowed);
            }
            for tree_entry in target_tree.iter() {
                let tree_name = tree_entry.name().unwrap();
                if tree_name == pkg_spec {
                    let pkg_object = tree_entry.to_object(&repo).unwrap();
                    pkgs.insert(pkg_spec, pkg_object);
                }
            }
        }

        // try!(validate_pkgs(&repo, &pkgs));

        let release = Release {
            repo: repo,
            target: target,
            notes: notes,
            pkgs: pkgs,
            NAMESPACE: namespace,
        };
        Ok(release)
    }

    fn existing_tags (&self) -> git2::References {
        self.repo.references_glob(self.NAMESPACE).unwrap()
    }

    pub fn tag_names (&self) -> HashSet<String> {
        let mut tags = HashSet::new();
        for (pkg_name, _) in self.pkgs.iter() {
            println!("{:?}", self.fmt_tag(&pkg_name))
        }
        tags
    }

    fn fmt_tag(&self, pkg_name: &str) -> String {
        let short_id = self.target.short_id().unwrap();
        let commit = short_id.as_str().unwrap();
        format!("{namespace}/{pkg_name}/{commit}",
                namespace=self.NAMESPACE,
                pkg_name=pkg_name,
                commit=commit)
    }


    pub fn unreleased (&self, pkg_name: &str, target: &git2::Object) -> Result<(), ReleaseError> {
        let glob = format!("refs/tags/{}/{}/*", self.NAMESPACE, pkg_name);
        for reference in self.repo.references_glob(&glob).unwrap() {
            if reference.target().unwrap() == target.id() {
                return Err(ReleaseError::AlreadyReleased);
            }
        }
        Ok(())
    }

    pub fn validate_unreleased (&self) -> Result<(), ReleaseError> {
        for (pkg_name, pkg_tree) in self.pkgs.iter() {
            try!(self.unreleased(pkg_name, pkg_tree));
        }
        Ok(())
    }

}

#[test]
fn validate_pkgs_behaviour () {
    assert!(false);
}
