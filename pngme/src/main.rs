mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use clap::{App, Arg};

fn main() {
    let matches = App::new("pngme")
        .arg(
            Arg::with_name("encode")
                .short("e")
                .long("encode")
                .help("encodes a secret message in a png file")
                .value_names(&["FILE", "CHUNK_TYPE", "Message"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("secret")
                .short("s")
                .long("secret")
                .help("gets the secret from a file")
                .value_names(&["FILE", "CHUNK_TYPE"])
                .takes_value(true),
        )
        .get_matches();

    let vals: Vec<&str> = args::get_match_value(&matches, "encode");
    if vals.len() == 3 {
        args::encode(&vals);
    }
    let secret: Vec<&str> = args::get_match_value(&matches, "secret");
    if secret.len() == 2 {
        let secret = match args::get_secret(&secret) {
            Ok(result) => result,
            Err(err) => err,
        };
        println!("{:?}", secret);
    }
}
