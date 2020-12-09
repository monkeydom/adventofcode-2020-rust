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
    part1();
    part2();
}

fn part1() {
    //let result = "None Yet";

    let numbers = parse_numbers(file::lines());
    let result = find_first_invalid(numbers, 25);

    aoc::print_solution1(format!("{:?}", result).as_str());
}

fn part2() {
    let result = "None Yet";
    aoc::print_solution2(format!("{:?} ", result).as_str());
}

fn find_first_invalid(numbers: Vec<i64>, window: usize) -> i64 {
    for location in window..numbers.len() {
        let mut tested_positive = false;
        // check number
        let test = numbers[location];
        'first: for first in (location - window)..(location - 1) {
            if numbers[first] > test {
                continue;
            }
            for second in first..location {
                if numbers[first] + numbers[second] == test {
                    tested_positive = true;
                    break 'first;
                }
            }
        }
        if !tested_positive {
            return test;
        }
    }
    return -1;
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

#[cfg(test)]
mod tests {

    use super::*;

    fn test_lines() -> impl Iterator<Item = String> {
        let source = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        source.split("\n").map(|l| l.to_string())
    }

    #[test]
    fn simple_test_1() {
        let numbers = parse_numbers(test_lines());
        assert_eq!(find_first_invalid(numbers, 5), 127);
    }
}
