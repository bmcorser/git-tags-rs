use yaml_rust::YamlEmitter;
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
        None          => return Err(LookupError::NotFound),
    };
    let result = match opts.value_of("number") {
        Some(number) => {
            let (number, packages) = lookup::channel_release(&repo, channel, number.parse::<u32>().unwrap()).unwrap();
            println!("channel: {:?}", channel);
            println!("number: {:?}", number);
            println!("packages:");
            for (path, tree) in packages {
                println!("  {:?}: {:?}", path, tree);
            }
        },
        None => {
            let (number, packages) = lookup::channel_latest(&repo, channel).unwrap();
            println!("channel: {:?}", channel);
            println!("number: {:?}", number);
            println!("packages:");
            for (path, tree) in packages {
                println!("  {:?}: {:?}", path, tree);
            }
        },
    };
    Ok(())
}
