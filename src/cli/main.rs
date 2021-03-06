#![feature(path_relative_from,collections,convert)]
#![allow(warnings)]
use std::result::Result;
use std::error::Error;
use std::convert::From;
use std::io;
extern crate tempfile;
extern crate clap;
extern crate git2;
extern crate yaml_rust;

extern crate tag;
use tag::error::ReleaseError;
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
    let result = match args.subcommand() {
        ("release", Some(cmd_args)) => match release::run(cmd_args) {
            Err(_) => {
                // println!("release BAD");
            },
            Ok(_)  => {
                // println!("yes fine");
            },
        },
        ("lookup", Some(cmd_args)) => match lookup::run(cmd_args) {
            Err(_) => {
                // println!("lookup BAD");
            },
            Ok(_)  => {
                // println!("yes fine");
            },
        },
        _ => (),
    };
    // println!("Bye.");
}
