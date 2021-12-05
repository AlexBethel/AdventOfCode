mod day1;
mod day2;

use std::env::args;

fn main() {
    let args = args().collect::<Vec<_>>();
    let n = if args.len() != 2 {
        eprintln!("Usage: aoc <day_number>");
        return;
    } else {
        args[1].parse::<usize>().unwrap()
    };

    match n {
        1 => day1::day1(),
        2 => day2::day2(),
        _ => {
            eprintln!("Invalid day number {}", n);
        }
    }
}
