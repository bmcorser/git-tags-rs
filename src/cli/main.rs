use std::result::Result;
use std::error::Error;
use std::convert::From;
use std::io;
extern crate tempfile;
extern crate clap;
extern crate git2;

extern crate tag;
mod release;
mod lookup;


fn main () -> () {
    let app = clap::App::new("git-tags")
                        .version("0.1.0")
                        .author("B M Corser <bmcorser@gmail.com>")
                        .about("Cut releases using Git tags.")
                     .arg_required_else_help(true)
                        .subcommand(release::command())
                        .subcommand(lookup::command());
    let args = app.get_matches();
    // let disaster: Result<(), Box<Error>> = Err(Box::new());
    let fail = Box::new(io::Error::new(io::ErrorKind::Other, "oh no!")) as Box<std::error::Error>;
    let result = match args.subcommand() {
        ("release", Some(cmd_args)) => release::run(cmd_args),
        // ("lookup", Some(cmd_args)) => lookup::run(cmd_args),
        _ => Err(fail),
    };
    println!("{:?}", result);
}
