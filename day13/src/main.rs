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

use std::cmp::Ordering;

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
    // let result = "None Yet";

    let mut result: i64 = -1;

    let line = file::lines().skip(1).next().unwrap();
    let mut indexed =
        line.split(",")
            .enumerate()
            .fold(vec![], |mut acc: Vec<(usize, i64)>, (i, s)| {
                if s != "x" {
                    acc.push((i, s.parse().unwrap()));
                }
                acc
            });

    indexed.sort_by(|(_, a), (_, b)| b.partial_cmp(&a).unwrap());
    println!("{:?}", &indexed);

    let mut iteration = 1i64;
    let mut max_sat= 2usize;
    loop {
        let start_baseline = 72;
        let candidate = (iteration * 379 + 228) * 557 - start_baseline;


        // numerical solution to
        // (x + 72) mod 557 = 0 && (x + 41) mod 379 = 0 && (x + 35) mod 37 =0 && (x + 70) mod 29 == 0 && (x+49) mod 23 == 0 && (x+91) mod 19 == 0 && (x + 58) mod 17 ==0 && (x+54) mod 13 == 0
        // from wolfram alpha
        // let candidate = 21875996124463 * iteration + 9205186551494;

        let satisfaction = indexed
        .iter()
        .filter(|(i, v)| (candidate + (*i as i64)) % v == 0)
        .count();
        // check the value
        if satisfaction == indexed.len() {
            result = candidate;
            break;
        }

        if satisfaction > max_sat {
            println!("{}: [{}/{}] {}", iteration, satisfaction, indexed.len(), candidate);
            max_sat = satisfaction;
        }
        iteration += 1;
    }

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
