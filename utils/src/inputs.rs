use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("file not found");
    io::BufReader::new(file).lines().map(|l| l.expect("failed to read line"))
}