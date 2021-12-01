mod day1;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Load a file that contains a list of integers into the
/// corresponding vector. Panic if anything goes wrong because I'm
/// lazy.
fn load_integers<I>(name: I) -> Vec<u32>
where
    I: AsRef<Path>,
{
    BufReader::new(File::open(name).unwrap())
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect()
}

fn main() {
    day1::day1();
}
