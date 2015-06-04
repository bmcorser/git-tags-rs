extern crate clap;
extern crate tempfile;

extern crate git2;

extern crate tag;
mod release;
mod lookup;

fn main () {
    let app = clap::App::new("git-tags")
                        .version("0.1.0")
                        .author("B M Corser <bmcorser@gmail.com>")
                        .about("Cut releases using Git tags.")
                     .arg_required_else_help(true)
                        .subcommand(release::command())
                        .subcommand(lookup::command());
    let args = app.get_matches();
    match args.subcommand() {
        ("release", Some(cmd_args)) => release::run(cmd_args),
        ("lookup", Some(cmd_args)) => lookup::run(cmd_args),
        _ => (),
    }
}
