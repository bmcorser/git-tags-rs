use std::path::Path;
use std::error::Error;
use std::collections::{HashMap, HashSet};

extern crate clap;
extern crate git2;

use tag::lookup;
use tag::error::LookupError;

pub fn command<'a, 'b, 'c, 'd, 'e, 'f> () -> clap::App<'a, 'b, 'c, 'd, 'e, 'f> {
    let options_usage = "\
    -l --list          'List all releases for package(s)'
    -r --repo=[repo]   'Specifiy the repository to release from'
    -y --yaml          'Output in YAML'";
    /*
    let alias_usage = "-a --alias=[alias] 'Optionally lookup by alias'";
    let alias_arg = clap::Arg::from_usage(alias_usage)
                              .conflicts_with("pkg");
    */
    clap::SubCommand::new("lookup")
                     .about("Get release tags by package name(s)")
                     .arg(clap::Arg::from_usage("<pkgs>.."))
                     .args_from_usage(options_usage)
                     // .arg(alias_arg)
                     .arg_required_else_help(true)
}

pub fn run(opts: &clap::ArgMatches) -> Result<(), LookupError> {
    let repo_path = Path::new(opts.value_of("repo").unwrap_or("."));
    let repo: git2::Repository = match git2::Repository::discover(repo_path) {
        Ok(repo) => repo,
        Err(err) => {
            println!("Git error: {:?}", err.to_string());
            return Err(LookupError::GitError);
        }
    };
    let pkg_names = opts.values_of("pkgs").unwrap();
    for pkg_name in pkg_names {
        let mut revwalk = repo.revwalk().unwrap();
        revwalk.push_head().unwrap();
    }

    for pkg_name in pkg_names {
        println!("{:?}", pkg_name);
        let glob = format!("refs/tags/releases/{}/*", pkg_name);
        let mut pkg_commits = HashSet::new();
        for reference in repo.references_glob(&glob).unwrap() {
            let mut split_ref = Vec::with_capacity(5);
            let ref_name = reference.name().unwrap();
            split_ref.extend(ref_name.split("/"));
            let commit = repo.revparse_single(split_ref[4]).unwrap();
        }
        let mut revwalk = repo.revwalk().unwrap();
        revwalk.push_head().unwrap();
        for reference in revwalk {
            println!("{:?}", reference);
        }
    }
    /**/

    Ok(())
}
