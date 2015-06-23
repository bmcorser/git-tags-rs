extern crate clap;
extern crate git2;
use std;
use std::error::Error;
use std::io::{Read, Seek, SeekFrom};
use std::collections::HashSet;
use std::path::Path;

use tag::release::Release;
use tag::error::ReleaseError;
use tempfile;

pub fn command<'a, 'b, 'c, 'd, 'e, 'f> () -> clap::App<'a, 'b, 'c, 'd, 'e, 'f> {
    clap::SubCommand::new("release")
                     .about("about release")
                     .arg(clap::Arg::from_usage("[channel] 'Which channel to release on (default `development`)'"))
                     .arg_required_else_help(true)
                     .args_from_usage("\
    -m --message=[message]  'Tell others what this release is'
    -c --commit=[commit]    'Release at a specific commit'
    -r --repo=[repo]        'Specifiy the repository to release from'
    -n --no-remote          'DEBUG: Don’t push tags to the remote'
    -f --force              'DEBUG: Ignore dirty repo warnings'")
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

pub fn run<'a> (opts: &'a clap::ArgMatches) -> Result<(), ReleaseError> {
    let repo_path = Path::new(opts.value_of("repo").unwrap_or("."));

    let channel = opts.value_of("channel").unwrap_or("development");
    let repo: git2::Repository = match git2::Repository::discover(repo_path) {
        Ok(repo) => repo,
        Err(err) => {
            println!("Git error: {:?}", err.to_string());
            return Err(ReleaseError::GitError);
        }
    };

    match repo.statuses(None).unwrap().len() {
        0 => (),
        _ => return Err(ReleaseError::DirtyWorkTree),
    }

    let release = Release::new(&repo, channel, None);
    match release {
        Ok(release) => {
            release.create_tag();
        },
        Err(err) => {
            match err {
                ReleaseError::DirtyWorkTree => {
                    // print status ooh colour
                    println!("{:?}: Untracked, uncommited or unadded files in working directory.", err);
                },
                ReleaseError::NoTrees => {
                    println!("{:?}: Nothing changed.", err);
                },
                _  => {
                    println!("urfkdm8: {:?}", err);
                }
            }
        }
    }
    let mut notes = String::new();
    match opts.value_of("message") {
        None    => notes = capture_message(notes),
        Some(m) => notes = m.to_string(),
    }
    Ok(())
}
