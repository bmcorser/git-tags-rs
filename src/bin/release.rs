use std;
extern crate clap;
extern crate tempfile;


pub fn command<'a, 'b, 'c, 'd, 'e, 'f> () -> clap::App<'a, 'b, 'c, 'd, 'e, 'f> {
    clap::SubCommand::new("release")
                     .about("about release")
                     // .arg(clap::Arg::from_usage("[pkg].."))
                     .arg_required_else_help(true)
                     .args_from_usage("\
    -m --message=[message]  'Tell others what this release is'
    -a --alias=[alias]      'Release named packages under an alias'
    -r --repo=[repo]        'Specifiy the repository to release from'
    -n --no-remote          'Don’t push tags to the remote'
    -f --force              'Make a release even if nothing changed'")
}

pub fn run(matches: clap::ArgMatches) {
    if let Some(ref message) = matches.value_of("message") {
        println!("{:?}",  message);
    } else {
        println!("{:?}", "nicht");
    }/*else {
        use std::io::{Read, Seek, SeekFrom};
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        std::process::Command::new("vim").arg(tmpfile.path())
            .spawn().unwrap().wait();
        tmpfile.seek(SeekFrom::Start(0)).unwrap();
        let mut buf = String::new();
        tmpfile.read_to_string(&mut buf).unwrap();
        println!("{:?}", buf);
    }*/
}
