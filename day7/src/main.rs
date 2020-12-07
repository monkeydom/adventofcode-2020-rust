mod aoc;
mod file;
// use itertools::Itertools;
// use ansi_term::Colour::Red;
// use std::{collections::HashSet, ops::RangeBounds};
// use std::collections::HashSet;

fn main() {
    aoc::preamble();
    part1();
    part2();
}

fn part1() {
    let result = "None Yet";
    aoc::print_solution1(format!("{}", result).as_str());
}

fn part2() {
    let result = "None Yet";
    aoc::print_solution2(format!("{} ", result).as_str());
}
