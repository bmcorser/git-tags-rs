use std::collections::HashSet;
use std::fmt;
use std::hash::Hasher;
use std::result::Result;
use std::path::PathBuf;
use std::error::Error;

use lookup;

use git2;

use error::{ReleaseError, LookupError};

pub struct Release<'a> {
    repo: &'a git2::Repository,
    pub target: git2::Object<'a>,
    pub channel: &'a str,
    pub number: u32,
    pub changed: HashSet<PathBuf>,
    pub ref_name: String,
    pub tag_name: String,
}

impl<'a> fmt::Debug for Release<'a> {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let short_id = self.target.short_id().unwrap();
        write!(f, "Release name {:?}", short_id.as_str().unwrap())
    }
}

impl<'a> Release<'a> {

    pub fn new (repo: &'a git2::Repository,
                channel: &'a str)
        -> Result<Release<'a>, ReleaseError> {
        let target = repo.revparse_single("HEAD").unwrap();
        let packages = lookup::packages(&repo).unwrap();
        let (prev_number, prev_packages) = match lookup::channel_latest(&repo, channel) {
            Ok((n, pkgs))  => (n, Some(pkgs)),
            Err(LookupError::EmptyChannel) => {
                println!("Opening a new channel, congrats!");
                (0, None)
            },
            Err(_) => {
                try!(repo.reset(&target, git2::ResetType::Hard, None));
                return Err(ReleaseError::Io)
            },
        };
        let mut packages_changed = HashSet::new();
        match prev_packages {
            Some(prev_packages) => {
                for (path, tree) in packages {
                    match prev_packages.get(&path) {
                        Some(prev_tree) => {
                            if *prev_tree != tree {
                                packages_changed.insert(path);
                            }
                        },
                        None => {
                            packages_changed.insert(path);
                        },
                    }
                }
            },
            None => {
                for (path, _) in packages {
                    packages_changed.insert(path);
                }
            },
        }

        if packages_changed.len() == 0 {
            try!(repo.reset(&target, git2::ResetType::Hard, None));
            return Err(ReleaseError::NoTrees);
        }
        let number = prev_number + 1;
        let tag_name = format!("releases/{}/{}", channel, number);
        let ref_name = format!("refs/tags/{}", tag_name);
        let target_clone = target.clone();
        let release = Release {
            repo: repo,
            target: target,
            channel: channel,
            changed: packages_changed,
            number: number,
            tag_name: tag_name,
            ref_name: ref_name,
        };
        try!(repo.reset(&target_clone, git2::ResetType::Hard, None));
        Ok(release)
    }

    pub fn create_tag (&self, notes: Option<&str>) -> Result<git2::Oid, git2::Error> {
        let signature = self.repo.signature().unwrap();
        let tag;
        match notes {
            Some(notes) => tag = self.repo.tag(&self.tag_name,
                                               &self.target,
                                               &signature,
                                               &notes,
                                               false).unwrap(),
            None => tag = self.repo.tag(&self.tag_name,
                                        &self.target,
                                        &signature,
                                        "",
                                        false).unwrap(),
        }
        Ok(tag)
    }

    pub fn push (&self) -> Result<(), git2::Error> {
        let mut origin = self.repo.find_remote("origin").unwrap();
        let mut push_origin = origin.push().unwrap();
        push_origin.add_refspec(&self.ref_name).unwrap();
        push_origin.finish().unwrap();
        match push_origin.statuses() {
            Ok(statuses) => {
                for push_status in statuses {
                    println!("{:?}", push_status.reference);
                    match push_status.message {
                        Some(message) => println!("{:?}", message),
                        None          => (),
                    }
                }
            },
            Err(err) => return Err(err),
        }
        Ok(())
    }
}

#[test]
fn validate_pkgs_behaviour () {
    assert!(false);
}
