use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);

    let mut contents = vec![];
    for line in buf_reader.lines() {
        contents.push(line?);
    }

    Ok(contents)
}