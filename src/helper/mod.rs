use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_file_line_iter(file: &str) -> impl Iterator<Item = String> {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);
    reader.lines().into_iter().map(|x| x.unwrap()).into_iter()
}
