extern crate docopt;

use docopt::Docopt;

static USAGE: &'static str = "
Lookup a release tag by package, commit or alias.

Usage:
  tag ship new <name>...
  tag ship <name> move <x> <y> [--speed=<kn>]
  tag ship shoot <x> <y>
  tag mine (set|remove) <x> <y> [--moored | --drifting]
  tag (-h | --help)
  tag --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --speed=<kn>  Speed in knots [default: 10].
  --moored      Moored (anchored) mine.
  --drifting    Drifting mine.
";

fn main() {
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    // You can conveniently access values with `get_{bool,count,str,vec}`
    // functions. If the key doesn't exist (or if, e.g., you use `get_str` on
    // a switch), then a sensible default value is returned.
    println!("\nSome values:");
    println!("  Speed: {}", args.get_str("--speed"));
    println!("  Drifting? {}", args.get_bool("--drifting"));
    println!("  Names: {:?}", args.get_vec("<name>"));
}
