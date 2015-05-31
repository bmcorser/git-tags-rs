extern crate clap;

use clap::{App, SubCommand};

mod release;

fn main () {
    let app = App::new("git-tags")
                  .version("0.1.0")
                  .author("B M Corser <bmcorser@gmail.com>")
                  .about("yaya")
                  .subcommand(release::command);
    let matches = app.get_matches();
    if let Some(matches) = matches.subcommand_matches("release") {
        if matches.is_present("message") {
            println!("{:?}",  matches.value_of("message"));
        } else {
            println!("No message");
        }
    }
}
