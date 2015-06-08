use std;
use std::error::Error;
use std::io::{Read, Seek, SeekFrom};
use std::collections::HashSet;
use std::path::Path;

use git2::Repository;

use tag::package::Package;
use tag::release::Release;
use tag::release;
use clap;
use tempfile;

pub fn command<'a, 'b, 'c, 'd, 'e, 'f> () -> clap::App<'a, 'b, 'c, 'd, 'e, 'f> {
    clap::SubCommand::new("release")
                     .about("about release")
                     .arg(clap::Arg::from_usage("<pkgs>... 'A sequence of package names'"))
                     .arg_required_else_help(true)
                     .args_from_usage("\
    -m --message=[message]  'Tell others what this release is'
    -a --alias=[alias]      'Release named packages under an alias'
    -c --commit=[commit]    'Release at a specific commit'
    -r --repo=[repo]        'Specifiy the repository to release from'
    -n --no-remote          'Don’t push tags to the remote'
    -f --force              'Ignore dirty repo warnings'")
}

#[allow(unused)]  // TODO: Use result
fn call_editor (tmpfile: &mut tempfile::NamedTempFile) -> () {
    std::process::Command::new("vim")
        .arg(tmpfile.path())
        .spawn().unwrap().wait();
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
}

fn capture_message<'a> (mut notes: String) -> String {
    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    call_editor(&mut tmpfile);
    tmpfile.read_to_string(&mut notes).unwrap();
    notes
}

pub fn run<'a> (opts: &'a clap::ArgMatches) -> Result<(), Box<Error>> {
    let repo_path = Path::new(opts.value_of("repo").unwrap_or("."));
    let repo = Repository::open(repo_path).unwrap();
    let mut notes = String::new();
    match opts.value_of("message") {
        None    => notes = capture_message(notes),
        Some(m) => notes = m.to_string(),
    }
    let commit = opts.value_of("commit").unwrap_or("HEAD");
    let mut pkgs = HashSet::new();
    for pkg_string in opts.values_of("pkgs").unwrap() {
        let pkg = try!(Package::new(&pkg_string));
        pkgs.insert(pkg);
    }
    let release = Release::new(
        &repo,
        commit,
        opts.value_of("alias"),
        pkgs,
        &notes,
        None
    );
    release.validate_tags().map_err(|e| println!("{:?}", e.to_string()));
    // let new_tags = release.new_tags();
    println!("{:?}", release);
    Ok(())
}
