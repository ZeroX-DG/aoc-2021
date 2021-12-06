use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input(file: &str) -> Vec<String> {
    let file = File::open(file).expect("Unable to read input file");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("Unable to read line"))
        .collect()
}
