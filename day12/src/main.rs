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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Location {
    x: i64,
    y: i64,
}

use std::ops::{Add, Mul, Sub};

impl Add<Location> for Location {
    type Output = Location;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Location> for Location {
    type Output = Location;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i64> for Location {
    type Output = Location;

    fn mul(self, rhs: i64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Location {
    fn new() -> Self {
        Location { x: 0, y: 0 }
    }

    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    fn directional_move(&mut self, dir: Direction, amount: i64) {
        use Direction::*;
        match dir {
            North => self.y += amount,
            South => self.y -= amount,
            West => self.x -= amount,
            East => self.x += amount,
        }
    }

    fn rotate_steps_around(&mut self, steps: i64, loc: &Location) {
        let unit = *self - *loc;
        match steps {
            2 | -2 => *self = *loc + (unit * -1),
            1 | -3 => *self = *loc + unit.r(),
            3 | -1 => *self = *loc + unit.l(),
            _ => (),
        }
    }

    fn l(&self) -> Self {
        Location {
            x: self.y * -1,
            y: self.x,
        }
    }

    fn r(&self) -> Self {
        Location {
            x: self.y,
            y: self.x * -1,
        }
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
        self.loc.directional_move(dir, amount);
    }

    fn manhattan_distance(&self) -> i64 {
        self.loc.manhattan_distance()
    }
}

#[derive(Debug)]
struct NavStateTwo {
    loc: Location,
    wp: Location,
}

impl NavStateTwo {
    fn new() -> Self {
        NavStateTwo {
            loc: Location::new(),
            wp: Location { x: 10, y: 1 },
        }
    }

    fn perform_line(&mut self, line: &str) {
        let amount: i64 = (line[1..])
            .parse()
            .expect("Needs to be a parseable integer");
        match line.chars().nth(0).unwrap() {
            'F' => {
                let unit = self.wp - self.loc;
                let movement = unit * amount;
                self.loc = self.loc + movement;
                self.wp = self.wp + movement;
            }
            c @ 'E' | c @ 'S' | c @ 'N' | c @ 'W' => {
                self.wp.directional_move(Direction::from_char(c), amount);
            }
            'L' => {
                self.wp.rotate_steps_around(amount / -90, &self.loc);
            }
            'R' => {
                self.wp.rotate_steps_around(amount / 90, &self.loc);
            }
            a => panic!("Unexpected instruction {}", a),
        };
    }

    fn manhattan_distance(&self) -> i64 {
        self.loc.manhattan_distance()
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

    let result = format!("{:?} => {}", ns, ns.manhattan_distance());

    aoc::print_solution1(format!("{:?}", result).as_str());
}

fn part2() {
    // let result = "None Yet";

    let mut ns = NavStateTwo::new();
    for line in file::lines() {
        ns.perform_line(&line[..]);
        //println!("{} -> {:?}", &line, ns)
    }

    let result = format!("{:?} => {}", ns, ns.manhattan_distance());
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
        assert_eq!(ns.manhattan_distance(), 25);
    }

    #[test]
    fn test_part2() {
        let lines: Vec<String> = test_lines().collect();
        println!("!{:?}", lines);
        let mut ns = NavStateTwo::new();
        println!("Starting point {:?}", ns);
        for line in test_lines() {
            ns.perform_line(&line[..]);
            println!("{} -> {:?}", &line, ns)
        }
        assert_eq!(ns.manhattan_distance(), 286);
    }

    #[test]
    fn test_location_rotation() {
        let loc = Location { x: 10, y: 1 };
        assert_eq!(loc, loc.r().l());
        assert_eq!(loc * -1, loc.r().r());
        assert_eq!(loc.l(), loc.r().r().r());
        assert_eq!(loc.r(), loc.l().l().l());
    }

    #[test]
    fn test_location_addition() {
        let loc = Location { x: 10, y: 1 };
        assert_eq!(loc + loc, loc * 2);
        assert_eq!(loc - loc, Location::new());
    }
}
