use std::path::Path;
use std::error::Error;
use std::collections::HashSet;
use std::collections::hash_map::{HashMap, Entry};

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
        let glob = format!("refs/tags/releases/{}/*", pkg_name);
        let mut pkg_commits = HashSet::new();
        let mut pkg_tags = HashMap::new();
        for reference in repo.references_glob(&glob).unwrap() {
            let ref_name = reference.name().unwrap();
            let commit = repo.revparse_single(
                ref_name.split("/").last().unwrap()).unwrap().id();
            pkg_commits.insert(commit);
            pkg_tags.insert(commit, reference.name().unwrap().to_string());
        }
        let mut revwalk = repo.revwalk().unwrap();
        revwalk.push_head().unwrap();
        revwalk.set_sorting(git2::SORT_TOPOLOGICAL);
        let mut latest: Option<String> = None;
        for commit in revwalk {
            match pkg_tags.entry(commit) {
                Entry::Occupied(ref_name) => {latest = Some(ref_name.get().clone()); break;},
                Entry::Vacant(_)          => (),
            }
        }
        match latest {
            Some(ref_name) => println!("package {:?}, latest tag: {:?}", pkg_name, ref_name),
            None         => (),
        }
    }
    Ok(())
}
