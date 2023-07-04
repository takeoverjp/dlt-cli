use std::ffi::OsString;

use clap::{arg, Command};

fn cli() -> Command {
    Command::new("dlt")
        .about("DLT Command Line Interface")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("log")
                .about("Store log")
                .arg(arg!(<MESSAGE> "The message to store"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("log", sub_matches)) => {
            println!(
                "Storing log \"{}\"",
                sub_matches.get_one::<String>("MESSAGE").expect("required")
            );
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {ext:?} with {args:?}");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }

    // Continued program logic goes here...
}
