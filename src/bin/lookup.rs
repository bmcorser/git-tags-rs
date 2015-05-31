extern crate clap;


pub fn command<'a, 'b, 'c, 'd, 'e, 'f> () -> clap::App<'a, 'b, 'c, 'd, 'e, 'f> {
    let usage = "
    -a --alias=<alias>      Optionally lookup by alias
    -r --repo=<repo>        Specifiy the repository to release from
    ";
    clap::SubCommand::new("lookup")
                     .about("about lookup")
                     .arg_from_usage(usage)
}
