//! Day 1.

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn part1(data: &[u32]) {
    println!("{}", data.windows(2).filter(|w| w[0] < w[1]).count());
}

pub fn part2(data: &[u32]) {
    let summed: Vec<u32> = data.windows(3).map(|w| w.iter().sum()).collect();
    part1(&summed)
}

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

pub fn day1() {
    let data = load_integers("day1.txt");
    part1(&data);
    part2(&data);
}
