use std;
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

#[allow(unused)]  // don’t want the actual return from the Command call
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


pub fn run(opts: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let repo_path = Path::new(opts.value_of("repo").unwrap_or("."));
    let repo = Repository::open(path).unwrap();
    let mut notes = String::new();
    match opts.value_of("message") {
        None    => notes = capture_message(notes),
        Some(m) => notes = m.to_string(),
    }
    let commit = opts.value_of("commit").unwrap_or("HEAD");
    let pkg_paths = opts.values_of("pkgs")
                        .unwrap()
                        .map(|string| Path::new(string))
                        .map(|path| Package::new(path).unwrap())
                        .collect();
    let abbrev_result = repo.revparse_single(commit)
                            .unwrap()
                            .short_id()
                            .unwrap();
    let release = Release::new(
        abbrev_result.as_str().unwrap(),
        opts.value_of("alias"),
        pkg_paths,
        &notes
    );
    println!("{:?}", release);
}
