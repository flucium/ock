// fn clap_command_

// fn clap_subcommand_
mod utils;

use std::io::{self,  stdout, Read, Write};

fn clap_command() -> clap::Command {
    const NAME: &str = "openck";

    const VERSION: &str = "0.0.1";

    const ABOUT: &str = "";

    const AUTHOR: &str = "flucium";

    clap::Command::new(NAME)
        .version(VERSION)
        .about(ABOUT)
        .author(AUTHOR)
        .subcommand(clap_subcommand_rand())
}

fn clap_subcommand_rand() -> clap::Command {
    const NAME: &str = "rand";

    const ABOUT: &str = "";

    clap::Command::new(NAME)
        .about(ABOUT)
        .args(clap_args_format())
}

fn clap_args_format() -> [clap::Arg; 3] {
    [
        clap::Arg::new("bytes")
            .long("bytes")
            .short('b')
            .required(false)
            .action(clap::ArgAction::SetTrue),
        clap::Arg::new("string")
            .long("string")
            .short('s')
            .required(false)
            .action(clap::ArgAction::SetTrue),
        clap::Arg::new("hex")
            .long("hex")
            // .short('h')
            .required(false)
            .action(clap::ArgAction::SetTrue),
        // clap::Arg::new("base64")
    ]
}



fn app() {
    let command = clap_command();

    let command_matches = command.get_matches();

    match command_matches.subcommand() {
        Some(("rand", rand_arg_matches)) => {
            let bytes = openck::rand::generate();

            if rand_arg_matches.args_present() {
                if rand_arg_matches.get_flag("bytes") {
                    //
                } else if rand_arg_matches.get_flag("string") {
                    //
                } else if rand_arg_matches.get_flag("hex") {
                    //
                    println!("{}", utils::bytes_to_hex(&bytes));
                } else {
                    //
                }
            }
        }
        _ => {
            print!("うぉ")
        }
    }
}


fn main() {
    app();
}
