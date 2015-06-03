extern crate clap;

pub fn command<'a, 'b, 'c, 'd, 'e, 'f> () -> clap::App<'a, 'b, 'c, 'd, 'e, 'f> {
    let usage = "\
    -r --repo=[repo]   'Specifiy the repository to release from'
    -y --yaml          'Output in YAML'";
    let alias_usage = "-a --alias=[alias] 'Optionally lookup by alias'";
    let alias_arg = clap::Arg::from_usage(alias_usage)
                              .conflicts_with("pkg");
    clap::SubCommand::new("lookup")
                     .about("about lookup")
                     .arg(clap::Arg::from_usage("[pkg].."))
                     .args_from_usage(usage)
                     .arg(alias_arg)
                     .arg_required_else_help(true)
}

pub fn run(matches: &clap::ArgMatches) {
}
