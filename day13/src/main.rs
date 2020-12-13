#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[allow(dead_code)]
mod aoc;
#[allow(dead_code)]
mod file;

// use itertools::Itertools;
// use ansi_term::Colour::Red;
// use std::{collections::HashSet, ops::RangeBounds};
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::fmt;

fn main() {
    aoc::preamble();
    // part1();
    part2();
}

fn part1() {
    let result = "None Yet";
    aoc::print_solution1(format!("{:?}", result).as_str());
}

fn part2() {
    let result = "None Yet";
    aoc::print_solution2(format!("{:?} ", result).as_str());
}

fn parse_numbers<T>(lines: T) -> Vec<i64>
where
    T: Iterator<Item = String>,
{
    lines
        .map(|l| l.parse::<i64>().expect("Needs to be a parseable number"))
        .collect()
}
// note
// this outputs standard io while running tests
// cargo test -- --nocapture

// also cool (needs cargo-watch installed)
// cargo watch -x "test -- --nocapture"

use indoc::indoc;

#[cfg(test)]
mod tests {

    use super::*;

    fn test_lines() -> impl Iterator<Item = String> {
        let source = indoc! {"939
        7,13,x,x,59,x,31,19"};
        source.split("\n").map(|l| l.to_string())
    }

    #[test]
    fn test_part1() {
        let lines: Vec<String> = test_lines().collect();
        println!("{:?}", lines);
        assert_eq!(true, false);
    }
}
