extern crate tempfile;
extern crate clap;
extern crate git2;

extern crate tag;
mod release;
mod lookup;


fn call_subcommand (matches: ArgMatches<'n, 'a>) -> Result<&str, Box<Error>> {
    match args.subcommand() {
        ("release", Some(cmd_args)) => println!("release"),
        ("lookup", Some(cmd_args)) => println!("lookup"),
        /*
        ("release", Some(cmd_args)) => release::run(cmd_args),
        ("lookup", Some(cmd_args)) => lookup::run(cmd_args),
        */
        _ => panic!("Aaargh!"),
    }
}

fn main () -> Result<&str, Box<Error>> {
    let app = clap::App::new("git-tags")
                        .version("0.1.0")
                        .author("B M Corser <bmcorser@gmail.com>")
                        .about("Cut releases using Git tags.")
                     .arg_required_else_help(true)
                        .subcommand(release::command())
                        .subcommand(lookup::command());
    call_subcommand(app.get_matches().subcommand())
}
