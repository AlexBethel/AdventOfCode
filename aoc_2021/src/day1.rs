//! Day 1.

use crate::load_integers;

pub fn part1(data: &[u32]) {
    println!("{}", data.windows(2).filter(|w| w[0] < w[1]).count());
}

pub fn part2(data: &[u32]) {
    let summed: Vec<u32> = data.windows(3).map(|w| w.iter().sum()).collect();
    part1(&summed)
}

pub fn day1() {
    let data = load_integers("day1.txt");
    part1(&data);
    part2(&data);
}
