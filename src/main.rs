use std::ffi::CString;
use std::ffi::OsString;

use clap::{arg, Command};

mod dlt {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

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
            let msg = sub_matches.get_one::<String>("MESSAGE").expect("required");
            let c_str = CString::new(format!("{}", msg)).unwrap();
            unsafe {
                dlt::dlt_log_init(dlt::DLT_LOG_TO_CONSOLE.try_into().unwrap());
                dlt::dlt_log(
                    dlt::DltLogLevelType_DLT_LOG_FATAL,
                    c_str.as_ptr().cast_mut(),
                );
            }
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
