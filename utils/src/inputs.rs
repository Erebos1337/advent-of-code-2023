use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines_impl<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("file not found");
    io::BufReader::new(file).lines().map(|l| l.expect("failed to read line"))
}

pub fn build_char_grid_impl<P>(path: P) -> Box<[Box<[char]>]>
where
    P: AsRef<Path>,
{
    read_lines_impl(path)
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Box<[char]>>())
        .collect::<Box<[Box<[char]>]>>()
}

/// Read lines from a file relative to the calling crate's directory.
/// Usage: `read_lines!("input.txt")`
#[macro_export]
macro_rules! read_lines {
    ($filename:expr) => {
        utils::inputs::read_lines_impl(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join($filename)
        )
    };
}

/// Build a 2D char grid from a file relative to the calling crate's directory.
/// Usage: `build_char_grid!("input.txt")`
#[macro_export]
macro_rules! build_char_grid {
    ($filename:expr) => {
        utils::inputs::build_char_grid_impl(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join($filename)
        )
    };
}