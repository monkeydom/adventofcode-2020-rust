#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[allow(dead_code)]
mod aoc;
#[allow(dead_code)]
mod file;

mod tests;

// use itertools::Itertools;
// use ansi_term::Colour::Red;
// use std::{collections::HashSet, ops::RangeBounds};
use std::collections::HashMap;
// use std::collections::HashSet;
// use std::fmt;

use std::cmp::Ordering;

fn main() {
    aoc::preamble();
    part1();
    part2();
}

fn part1() {
    // let result = "None Yet";
    let mem = process_lines(file::lines());
    let result = sum_memory(&mem);
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

fn sum_memory(mem: &HashMap<usize, u64>) -> u64 {
    mem.iter().fold(0u64, |acc, (_, v)| acc + *v)
}

fn parse_mask(mask: &str) -> (u64, u64) {
    let mut set_mask = 0u64;
    let mut clear_mask = 0u64;

    for (loc, c) in mask.chars().enumerate() {
        match c {
            '1' => set_mask |= 1u64 << (35 - loc),
            '0' => clear_mask |= 1u64 << (35 - loc),
            _ => (),
        }
    }

    (set_mask, clear_mask)
}

fn process_lines(lines: impl Iterator<Item = String>) -> HashMap<usize, u64> {
    let mut result = HashMap::new();

    let mut set_mask = 0u64;
    let mut clear_mask = 0u64;

    for line in lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        match parts[0] {
            "mask" => {
                let pm = parse_mask(parts[1]);
                set_mask = pm.0;
                clear_mask = pm.1;
            }

            _ => {
                let loc: usize = parts[0][4..(parts[0].len() - 1)].parse().unwrap();
                let value: u64 = parts[1].parse().unwrap();
                let value = value | set_mask;
                let value = value & !clear_mask;
                result.insert(loc, value);
            }
        }
    }

    result
}
