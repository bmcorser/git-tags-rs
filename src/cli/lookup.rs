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
    -r --repo=[repo]   'Specifiy the repository to release from'";
    /*
    let alias_usage = "-a --alias=[alias] 'Optionally lookup by alias'";
    let alias_arg = clap::Arg::from_usage(alias_usage)
                              .conflicts_with("pkg");
    */
    clap::SubCommand::new("lookup")
                     .about("Get release tags by package name(s)")
                     .arg(clap::Arg::from_usage("<channel>"))
                     .arg(clap::Arg::from_usage("[number]"))
                     .args_from_usage(options_usage)
                     // .arg(alias_arg)
                     .arg_required_else_help(true)
}

pub fn run(opts: &clap::ArgMatches) -> Result<(), LookupError> {
    let repo_path = Path::new(opts.value_of("repo").unwrap_or("."));
    let repo: git2::Repository = try!(git2::Repository::discover(repo_path));
    let channel = match opts.value_of("channel") {
        Some(channel) => channel,
        None          => return Err(LookupError::NoChannel),
    };
    let packages = lookup::packages(&repo).unwrap();
    for (name, id) in packages {
        // println!("{:?}\t{:?}", name, id);
    }
    let channel_releases = lookup::channel_latest(&repo, "development").unwrap();
    /*
    for release in channel_releases {
        println!("{:?}", release);
    }
    */

    let glob = format!("refs/tags/releases/{}/*", channel);
    /*
    let mut pkg_commits = HashSet::new();
    let mut pkg_tags = HashMap::new();
    let mut split_ref = Vec::with_capacity(5);
    for reference in repo.references_glob(&glob).unwrap() {
        let ref_name = reference.name().unwrap();
        split_ref.extend(ref_name.split("/"));
        ref_name.split("/").unwrap()[2];
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
    */
    Ok(())
}
