extern crate clap;

use clap::{App, SubCommand};

static DUMMY_USAGE: &'static str = "-m --message=[message] 'Tell others about your release'";

fn main() {
    let release_command = SubCommand::new("release")
                                     .about("about release")
                                     .arg_from_usage(DUMMY_USAGE);
    let app = App::new("git-tags")
                  .version("0.1.0")
                  .author("B M Corser <bmcorser@gmail.com>")
                  .about("yaya")
                  .subcommand(release_command);
    let matches = app.get_matches();
    if let Some(matches) = matches.subcommand_matches("release") {
        println!("{:?}",  matches.value_of("message"));
    }
    // println!("{:?}", matches);
}
