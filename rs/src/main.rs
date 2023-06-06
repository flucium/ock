use std::io::{self, Read, Write};

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
            clap::Command::new("aes-128-gcm").args(app_symmetric_args()),
            clap::Command::new("aes-192-gcm").args(app_symmetric_args()),
            clap::Command::new("aes-256-gcm").args(app_symmetric_args()),
            clap::Command::new("chacha20-poly1305").args(app_symmetric_args()),
        ])
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

fn stdout(bytes: &[u8]) {
    if let Err(err) = io::stdout().write_all(bytes) {
        panic!("{err}")
    }
}

fn stderr(bytes: &[u8]) {
    if let Err(err) = io::stderr().write_all(bytes) {
        panic!("{err}")
    }
}

/*

*/

fn main() {
    let app = app();

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("aes-128-gcm", arg_matches)) => {
            let key = arg_matches
                .get_one("key")
                .unwrap_or(&String::default())
                .to_owned();

            let message = arg_matches
                .get_one("message")
                .unwrap_or(&String::default())
                .to_owned();

            if arg_matches.get_flag("encrypt") {
                let c = ock::symmetric::Symmetric::Aes128Gcm {
                    key: Box::new(key.as_bytes().try_into().unwrap()),
                    aad: b"",
                    data: message.as_bytes(),
                }
                .encrypt()
                .unwrap();

                println!("{:?}", c);
            } else if arg_matches.get_flag("decrypt") {
            } else {
            }
        }
        Some(("aes-192-gcm", arg_matches)) => {}
        Some(("aes-256-gcm", arg_matches)) => {}
        Some(("chacha20-poly1305", arg_matches)) => {}
        _ => {}
    }
}

/*
    Encrypt : encrypt enc e
    Decrypt : decrypt dec d
    Output  : output out o
    Input   : input in i
    File    : file f
    Length  : length len l
    Key     : key k
    Password: password psw p
    Generate: generate gen g


    --AES--
    aes-128-gcm
    aes-192-gcm
    aes-256-gcm

    --ChaCha--
    chacha20-poly1305

    --Random--
    random

*/

// clap::Arg::new("password")
//     .long("password")
//     .short('p')
//     .alias("psw");

// clap::Arg::new("generate")
//     .long("generate")
//     .short('g')
//     .alias("gen");

// clap::Arg::new("key").long("key").short('k');

// clap::Arg::new("length")
//     .long("length")
//     .short('l')
//     .alias("len");

// clap::Arg::new("file").long("file").short('f');

// clap::Arg::new("input").long("input").short('i').alias("in");

// clap::Arg::new("output")
//     .long("output")
//     .short('o')
//     .alias("out");

// clap::Arg::new("decrypt")
//     .long("decrypt")
//     .short('d')
//     .alias("dec");

// clap::Arg::new("encrypt")
//     .long("encrypt")
//     .short('e')
//     .alias("enc");
