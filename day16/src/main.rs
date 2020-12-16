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
    //    let result = "None Yet";
    let result = solve_part1(file::lines());
    aoc::print_solution1(format!("{:?}", result).as_str());
}

fn part2() {
    let result = "None Yet";
    aoc::print_solution2(format!("{:?} ", result).as_str());
}

fn is_valid(fields: &Vec<Field>, n: i64) -> bool {
    for range in fields.iter().flat_map(|f| f.ranges.iter()) {
        if range.contains(&n) {
            return true;
        }
    }
    false
}

fn solve_part1(lines: impl Iterator<Item = String>) -> i64 {
    let (fields, _, tickets) = process_lines(lines);
    tickets.iter().flat_map(|t| t.iter()).fold(
        0,
        |acc, n| {
            if is_valid(&fields, *n) {
                acc
            } else {
                acc + n
            }
        },
    )
}

use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug)]
struct Field {
    name: String,
    ranges: Vec<RangeInclusive<i64>>,
}

impl FromStr for Field {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split1: Vec<&str> = s.split(": ").collect();
        let name = split1[0].to_string();
        let split: Vec<&str> = split1[1].split(" or ").collect();
        let mut ranges = vec![];
        for split_line in split {
            let sr: Vec<&str> = split_line.split("-").collect();
            let a: i64 = sr[0].parse()?;
            let b: i64 = sr[1].parse()?;
            ranges.push(a..=b);
        }

        Ok(Field { name: name, ranges })
    }
}

fn process_lines(lines: impl Iterator<Item = String>) -> (Vec<Field>, Vec<i64>, Vec<Vec<i64>>) {
    let mut my_ticket: Vec<i64> = vec![];
    let mut nearby_tickets: Vec<Vec<i64>> = vec![];
    let mut fields: Vec<Field> = vec![];
    let mut state = 0;
    for line in lines {
        if line == "" {
            state += 1;
            continue;
        }
        println!("{:?}", &line);
        if state == 0 {
            fields.push(Field::from_str(&line).unwrap());
        } else if line.chars().last().unwrap() != ':' {
            let pass: Vec<i64> = line.split(",").map(|n| n.parse::<i64>().unwrap()).collect();
            if state == 1 {
                my_ticket = pass;
            } else {
                nearby_tickets.push(pass);
            }
        }
    }
    println!("{:?} {:?} {:?}", fields, my_ticket, nearby_tickets);
    (fields, my_ticket, nearby_tickets)
}
