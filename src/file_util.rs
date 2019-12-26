use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

pub fn read(path: &str) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut file = File::open(path)?;

    let _ = file.read_to_string(&mut buffer);
    Ok(buffer)
}
