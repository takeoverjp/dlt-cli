use std::ffi::CString;
use std::ffi::OsString;

use clap::{arg, Command};
use libdlt_sys::dlt_user_log_write_finish;
use libdlt_sys::dlt_user_log_write_string;
use libdlt_sys::DltContext;
use libdlt_sys::DltLogLevelType;
use libdlt_sys::*;

fn cli() -> Command {
    Command::new("dlt")
        .about("DLT Command Line Interface")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("log")
                .about("Store log")
                .arg(
                    arg!(<MESSAGE> "The message to store")
                        .value_parser(clap::builder::NonEmptyStringValueParser::new()),
                )
                .arg_required_else_help(true)
                .arg(
                    arg!(-c --"context-id" <CONTEXT_ID>)
                        .value_parser(clap::builder::NonEmptyStringValueParser::new())
                        .default_value("DLTC"),
                ),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("log", sub_matches)) => {
            let msg: &String = sub_matches.get_one("MESSAGE").expect("required");
            let ctid: &String = sub_matches.get_one("context-id").expect("required");
            let message = CString::new(msg.to_string()).unwrap();
            unsafe {
                let app_name = CString::new("APP").unwrap();
                dlt_register_app(
                    app_name.as_ptr(),
                    CString::new("Example Application").unwrap().as_ptr(),
                );
                let mut context = DltContext::new_uninitialized();
                let context_id = CString::new(ctid.clone()).unwrap();
                let description = CString::new("This is a longer description").unwrap();
                dlt_register_context(
                    context.as_mut_ptr(),
                    context_id.as_ptr(),
                    description.as_ptr(),
                );
                let mut local_context = DltContextData::new_uninitialized();

                let _dlt_local = dlt_user_log_write_start_id(
                    context.as_mut_ptr(),
                    local_context.as_mut_ptr(),
                    DltLogLevelType::DLT_LOG_ERROR,
                    1234,
                );
                dlt_user_log_write_string(local_context.as_mut_ptr(), message.as_ptr());
                dlt_user_log_write_finish(local_context.as_mut_ptr());
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
