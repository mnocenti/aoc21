
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn read_lines(path: &str) -> MyResult<impl Iterator<Item=String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map(Result::unwrap))
}