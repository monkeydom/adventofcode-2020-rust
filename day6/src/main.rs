use itertools::Itertools;
mod aoc;
mod file;
// use ansi_term::Colour::Red;
// use std::{collections::HashSet, ops::RangeBounds};
use std::collections::HashSet;


fn main() {
    aoc::preamble();
    part1();
    part2();
}

fn part1() {
    let mut chars: HashSet<char> = HashSet::new();

let mut result = 0;

    for (i, section) in file::sections().enumerate() {
        let mut count = 0;
        for line in section {
            for c in line.chars() {
                chars.insert(c);
            }
            count += 1;
        }
        result += chars.len();
        println!("{}: {} lines {:?}", i, count, chars.iter().sorted().collect::<String>());
        chars.drain();
    }
    aoc::print_solution1(format!("{} combined yes answers", result).as_str());
}

fn part2() {
    aoc::print_solution2(format!("{}", "none yet").as_str());
}
