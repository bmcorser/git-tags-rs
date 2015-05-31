extern crate clap;

use clap::{App, SubCommand};

static RELEASE_USAGE: &'static str = "
-m --message=<message>  Tell others what this release is

-a --alias=<alias>      Release named packages under an alias

-r --repo=<repo>        Specifiy the repository to release from [default: ./]

-n --no-remote          Don’t push tags to the remote

-f --force              Make a release even if nothing changed
";

fn main() {
    let release_command = SubCommand::new("release")
                                     .about("about release")
                                     .arg_from_usage(RELEASE_USAGE);
    let app = App::new("git-tags")
                  .version("0.1.0")
                  .author("B M Corser <bmcorser@gmail.com>")
                  .about("yaya")
                  .subcommand(release_command);
    let matches = app.get_matches();
    if let Some(matches) = matches.subcommand_matches("release") {
        println!("{:?}",  matches.value_of("message"));
    }
}
