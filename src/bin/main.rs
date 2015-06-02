extern crate clap;
extern crate tempfile;

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
    let root_matches = app.get_matches();
    match root_matches.subcommand() {
        ("release", Some(matches)) => release::run(matches),
        ("lookup", Some(matches)) => release::run(matches),
        _ => (),
    }
}
