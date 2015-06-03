use std;
use std::collections::HashSet;

extern crate clap;
extern crate tempfile;
extern crate tag;
use tag;



pub fn command<'a, 'b, 'c, 'd, 'e, 'f> () -> clap::App<'a, 'b, 'c, 'd, 'e, 'f> {
    tag;
    clap::SubCommand::new("release")
                     .about("about release")
                     .arg(clap::Arg::from_usage("<pkgs>... 'A sequence of package names'"))
                     .arg_required_else_help(true)
                     .args_from_usage("\
    -m --message=[message]  'Tell others what this release is'
    -a --alias=[alias]      'Release named packages under an alias'
    -r --repo=[repo]        'Specifiy the repository to release from'
    -n --no-remote          'Don’t push tags to the remote'
    -f --force              'Make a release even if nothing changed'")
}

fn capture_message<'a> (mut message: String) -> String {
    use std::io::{Read, Seek, SeekFrom};
    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    std::process::Command::new("vim")
        .arg(tmpfile.path())
        .spawn().unwrap().wait();
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    tmpfile.read_to_string(&mut message).unwrap();
    message
}

pub fn run(matches: &clap::ArgMatches) {
    let mut message = String::new();
    match matches.value_of("message") {
        None    => (), //message = capture_message(message),
        Some(m) => message = m.to_string(),
    }
    let pkgs = HashSet::new();
    for pkg in matches.values_of("pkgs") {
        pkgs.insert(tag::lib::Package.new(pkg))
    }
}
