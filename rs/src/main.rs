use aead::KeyInit;

const NAME: &str = "openck";

const VERSION: &str = "0.0.1";

const AUTHOR: &str = "flucium <flucium@flucium.net>";

const ABOUT: &str = "";

fn app() -> clap::Command {
    clap::Command::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
}

fn main() {
    /*
        --encrypt -e
        --decrypt -d
        --input -i
        --output -o
        --file -f
    */
    
    
    app();   
}