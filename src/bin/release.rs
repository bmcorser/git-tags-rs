extern crate getopts;
use getopts::Options;
use std::env;


USAGE = "
git-tags-rs

Usage:
  tag-release [<pkg>...] [-m <message>] [-a <alias>] [-r <repo>] [-n] [-f] [--help]

Options:

  -m <message> 
  --message=<message>  Tell others what this release is

  -a <alias>
  --alias=<alias>      Release named packages under an alias

  -r <repo>
  --repo=<repo>        Specifiy the repository to release from [default: ./]

  -n
  --no-remote          Don’t push tags to the remote

  -f
  --force              Make a release even if nothing changed

  -h
  --help               Print this help message
";

#[derive(RustcDecodable)]
struct Args {
    arg_pkg: Vec<String>,
    flag_message: String,
    flag_alias: String,
    flag_repo: String,
    flag_no_remote: bool,
    flag_force: bool,
    flag_help: bool,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
}
