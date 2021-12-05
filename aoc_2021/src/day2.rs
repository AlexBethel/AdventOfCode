//! Day 2.

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn part1(data: &[((i32, i32), i32)]) {
    let mut horiz = 0;
    let mut depth = 0;

    for ((dir_horiz, dir_vert), dist) in data {
        horiz += dir_horiz * dist;
        depth += dir_vert * dist;
    }

    println!("{:?}", (horiz, depth));
    println!("{}", horiz * depth);
}

fn part2(data: &[((i32, i32), i32)]) {
    let mut horiz = 0;
    let mut depth = 0;

    let mut aim = 0;

    for ((dir_horiz, dir_aim), dist) in data {
        horiz += dir_horiz * dist;
        depth += aim * dir_horiz * dist;
        aim += dir_aim * dist;
    }

    println!("{:?}", (horiz, depth));
    println!("{}", horiz * depth);
}

fn direction(name: &str) -> (i32, i32) {
    match name {
        "forward" => (1, 0),
        "down" => (0, 1),
        "up" => (0, -1),
        _ => panic!("Invalid direction {}", name),
    }
}

fn load_file(name: impl AsRef<Path>) -> Vec<((i32, i32), i32)> {
    BufReader::new(File::open(name).unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let words = line.split_whitespace().collect::<Vec<_>>();
            assert_eq!(words.len(), 2);

            (direction(words[0]), words[1].parse().unwrap())
        })
        .collect()
}

pub fn day2() {
    let data = load_file("day2.txt");
    part1(&data);
    part2(&data);
}
