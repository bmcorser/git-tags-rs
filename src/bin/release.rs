extern crate clap;


const usage: &'static str = "
-m --message=<message>  Tell others what this release is
-a --alias=<alias>      Release named packages under an alias
-r --repo=<repo>        Specifiy the repository to release from
-n --no-remote          Don’t push tags to the remote
-f --force              Make a release even if nothing changed
";

pub const command: clap::App<'static, 'static, 'static, 'static, 'static, 'static> = 
    clap::SubCommand::new("release")
                     .about("about release")
                     .arg_from_usage(usage);
