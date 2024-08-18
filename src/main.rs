mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use std::str::FromStr;

use chunk_type::ChunkType;

fn main() {
    // comment
    let ct = ChunkType::from_str("Ru1t");
    println!("{}", ct.is_err());
}
