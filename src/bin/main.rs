extern crate clap;

use clap::{App, SubCommand};

mod release;
mod lookup;

fn main () {
    let app = App::new("git-tags")
                  .version("0.1.0")
                  .author("B M Corser <bmcorser@gmail.com>")
                  .about("Cut releases using Git tags.")
                  .subcommand(release::command())
                  .subcommand(lookup::command());
    let matches = app.get_matches();
    if let Some(matches) = matches.subcommand_matches("release") {
        if matches.is_present("message") {
            println!("{:?}",  matches.value_of("message"));
        } else {
            println!("No message");
        }
    }
}
