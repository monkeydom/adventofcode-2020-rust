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

#[derive(Debug)]
struct Location {
    x: i64,
    y: i64,
}

impl Location {
    fn new() -> Self {
        Location { x: 0, y: 0 }
    }

    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North = 0,
    East,
    South,
    West,
}

impl Direction {
    fn from_char(c: char) -> Self {
        use Direction::*;
        match c {
            'N' => North,
            'E' => East,
            'S' => South,
            'W' => West,
            _ => East,
        }
    }

    fn r(&mut self) {
        use Direction::*;
        *self = match *self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn l(&mut self) {
        use Direction::*;
        *self = match *self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn turn(&mut self, amount: i64) {
        if amount > 0 {
            for _ in 0..(amount / 90) {
                self.r();
            }
        } else {
            for _ in 0..(amount / -90) {
                self.l();
            }
        }
    }
}

#[derive(Debug)]
struct NavState {
    loc: Location,
    dir: Direction,
}

impl NavState {
    fn new() -> Self {
        NavState {
            loc: Location::new(),
            dir: Direction::East,
        }
    }

    fn perform_line(&mut self, line: &str) {
        let amount: i64 = (line[1..])
            .parse()
            .expect("Needs to be a parseable integer");
        let (dir, amount) = match line.chars().nth(0).unwrap() {
            'F' => (self.dir, amount),
            c @ 'E' | c @ 'S' | c @ 'N' | c @ 'W' => (Direction::from_char(c), amount),
            'L' => {
                self.dir.turn(-amount);
                (Direction::East, 0)
            }
            'R' => {
                self.dir.turn(amount);
                (Direction::East, 0)
            }
            a => panic!("Unexpected instruction {}", a),
        };
        self.r#move(dir, amount);
    }

    fn r#move(&mut self, dir: Direction, amount: i64) {
        use Direction::*;
        match dir {
            North => self.loc.y += amount,
            South => self.loc.y -= amount,
            West => self.loc.x -= amount,
            East => self.loc.x += amount,
        }
    }
}

fn main() {
    aoc::preamble();
    part1();
    part2();
}

fn part1() {
    // let result = "None Yet";
    let mut ns = NavState::new();
    for line in file::lines() {
        ns.perform_line(&line[..]);
        //println!("{} -> {:?}", &line, ns)
    }

    let result = format!("{:?} => {}", ns, ns.loc.manhattan_distance());

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
        let source = indoc! {"F10
        N3
        F7
        R90
        F11"};
        source.split("\n").map(|l| l.to_string())
    }

    #[test]
    fn test_part1() {
        let lines: Vec<String> = test_lines().collect();
        println!("!{:?}", lines);
        let mut ns = NavState::new();
        for line in test_lines() {
            ns.perform_line(&line[..]);
            println!("{} -> {:?}", &line, ns)
        }
        assert_eq!(ns.loc.manhattan_distance(), 25);
    }
}
