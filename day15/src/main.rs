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
// use std::cmp::Ordering;

fn main() {
    aoc::preamble();
    part1();
    part2();
}

fn part1() {
    // let result = "None Yet";
    let result = nth_number(vec![17, 1, 3, 16, 19, 0], 2020);
    aoc::print_solution1(format!("{:?}", result).as_str());
}

fn part2() {
    let result = "None Yet";
    // let result = "None Yet";
    let result = nth_number(vec![17, 1, 3, 16, 19, 0], 30_000_000);
    aoc::print_solution2(format!("{:?} ", result).as_str());
}

fn nth_number(starters: Vec<i64>, final_number: i64) -> i64 {
    let mut turn = 1i64;
    let mut last_occurance = HashMap::new();
    let mut number = 0;

    for n in starters {
        last_occurance.insert(n, turn);
        number = n;
        println!("{}: {}", turn, number);
        turn += 1;
    }

    loop {
        let prev_number = number;
        number = match last_occurance.get(&number) {
            Some(prev_turn) => turn - prev_turn - 1,
            None => 0,
        };
        last_occurance.insert(prev_number, turn - 1);

        // println!("{}: {} {:?}", turn, number, &last_occurance);

        if turn == final_number {
            return number;
        }
        turn += 1;
    }
}
