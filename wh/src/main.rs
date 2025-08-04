use clap::{Arg, Command};
use clap::ArgAction::SetTrue;

fn main() {
    let matches = Command::new("clap_parser")
        .arg(
            Arg::new("global")
                .long("global")
                .action(SetTrue)
                .help("A binary flag"),
        )
        .arg(
            Arg::new("bin")
                .required(true)
                .index(1)
                .help("A single positional argument"),
        )
        .get_matches();


    let global = matches.get_flag("global");
    let bin : &String = matches.get_one("bin").unwrap();

    println!("Global flag: {}", global);
    println!("Bin argument: {}", bin);
    let res = if global {
        which::which_global(bin)
    } else {
        which::which(bin)
    };
    let path = res.expect("Couldn't find bin");
    println!("{:?}", path );
}