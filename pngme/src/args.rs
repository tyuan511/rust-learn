use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use clap::ArgMatches;
use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

pub fn encode(vals: &[&str]) {
    let mut image = Png::try_from(fs::read(vals[0]).unwrap().as_ref()).unwrap();
    let msg: Vec<u8> = vals[2].bytes().collect();
    image.append_chunk(Chunk::new(ChunkType::from_str(vals[1]).unwrap(), msg));
    let result = image.as_bytes();
    fs::write(vals[0], result).unwrap();
}

pub fn get_secret(vals: &[&str]) -> Result<String, String> {
    let image = Png::try_from(fs::read(vals[0]).unwrap().as_ref()).unwrap();
    let secret = match image.chunk_by_type(vals[1]) {
        Some(result) => result,
        None => return Err(String::from("chunk not found")),
    };
    Ok(secret.data_as_string().unwrap())
}

pub fn get_match_value<'a>(matches: &'a ArgMatches, arg: &'a str) -> Vec<&'a str> {
    match matches.values_of(arg) {
        Some(result) => result.collect(),
        None => vec![],
    }
}
