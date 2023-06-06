use ock::{self, size::*};
use std::{
    self,
    io::{stderr, Write},
    process::exit,
};
const NAME: &str = "OCK";

const VERSION: &str = "0.0.1";

const AUTHOR: &str = "flucium <flucium@flucium.net>";

const ABOUT: &str = "";

fn app() -> clap::Command {
    clap::Command::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .subcommands([
            clap::Command::new("aes-128-gcm")
                .args(app_symmetric_args())
                .args(app_format_args())
                .args(app_compress_args()),
            clap::Command::new("aes-192-gcm")
                .args(app_symmetric_args())
                .args(app_format_args())
                .args(app_compress_args()),
            clap::Command::new("aes-256-gcm")
                .args(app_symmetric_args())
                .args(app_format_args())
                .args(app_compress_args()),
            clap::Command::new("chacha20-poly1305")
                .args(app_symmetric_args())
                .args(app_format_args())
                .args(app_compress_args()),
        ])
}

fn app_compress_args() -> [clap::Arg; 1] {
    [clap::Arg::new("deflate")
        .long("deflate")
        .action(clap::ArgAction::SetTrue)]
}

fn app_format_args() -> [clap::Arg; 2] {
    [
        clap::Arg::new("base64")
            .long("base64")
            .alias("b64")
            .action(clap::ArgAction::SetTrue),
        clap::Arg::new("hex")
            .long("hex")
            .action(clap::ArgAction::SetTrue),
    ]
}

fn app_symmetric_args() -> [clap::Arg; 4] {
    [
        clap::Arg::new("encrypt")
            .long("encrypt")
            .short('e')
            .alias("enc")
            .action(clap::ArgAction::SetTrue),
        clap::Arg::new("decrypt")
            .long("decrypt")
            .short('d')
            .alias("dec")
            .action(clap::ArgAction::SetTrue),
        clap::Arg::new("key")
            .long("key")
            .short('k')
            .action(clap::ArgAction::Set)
            .required(false),
        clap::Arg::new("message")
            .long("message")
            .short('m')
            .alias("msg")
            .action(clap::ArgAction::Set)
            .required(false),
    ]
}

fn main() -> std::io::Result<()> {
    let app = app();

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("aes-128-gcm", arg_matches)) => {
            let _key: &[u8; SIZE_U16] = match arg_matches
                .get_one::<String>("key")
                .unwrap_or(&String::default())
                .as_bytes()
                .try_into()
            {
                Err(err) => {
                    write!(&mut stderr(), "{:?}", err)?;

                    exit(1)
                }
                Ok(key) => key,
            };

            let _message = arg_matches
                .get_one::<String>("message")
                .unwrap_or(&String::default())
                .as_bytes();

            if arg_matches.get_flag("encrypt") {}
            if arg_matches.get_flag("decrypt") {
            } else {
            }
        }
        Some(("aes-192-gcm", arg_matches)) => {
            let _key: &[u8; SIZE_U24] = match arg_matches
                .get_one::<String>("key")
                .unwrap_or(&String::default())
                .as_bytes()
                .try_into()
            {
                Err(err) => {
                    write!(&mut stderr(), "{:?}", err)?;

                    exit(1)
                }
                Ok(key) => key,
            };

            let _message = arg_matches
                .get_one::<String>("message")
                .unwrap_or(&String::default())
                .as_bytes();

            if arg_matches.get_flag("encrypt") {}
            if arg_matches.get_flag("decrypt") {
            } else {
            }
        }
        Some(("aes-256-gcm", arg_matches)) => {
            let _key: &[u8; SIZE_U32] = match arg_matches
                .get_one::<String>("key")
                .unwrap_or(&String::default())
                .as_bytes()
                .try_into()
            {
                Err(err) => {
                    write!(&mut stderr(), "{:?}", err)?;

                    exit(1)
                }
                Ok(key) => key,
            };

            let _message = arg_matches
                .get_one::<String>("message")
                .unwrap_or(&String::default())
                .as_bytes();

            if arg_matches.get_flag("encrypt") {}
            if arg_matches.get_flag("decrypt") {
            } else {
            }
        }
        Some(("chacha20-poly1305", arg_matches)) => {
            let _key: &[u8; SIZE_U32] = match arg_matches
                .get_one::<String>("key")
                .unwrap_or(&String::default())
                .as_bytes()
                .try_into()
            {
                Err(err) => {
                    write!(&mut stderr(), "{:?}", err)?;

                    exit(1)
                }
                Ok(key) => key,
            };

            let _message = arg_matches
                .get_one::<String>("message")
                .unwrap_or(&String::default())
                .as_bytes();

            if arg_matches.get_flag("encrypt") {}
            if arg_matches.get_flag("decrypt") {
            } else {
            }
        }
        _ => {}
    }

    Ok(())
}
