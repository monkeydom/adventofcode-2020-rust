#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[allow(dead_code)]
mod aoc;
#[allow(dead_code)]
mod file;

// use itertools::Itertools;
// use ansi_term::Colour::Red;
// use std::{collections::HashSet, ops::RangeBounds};
use std::collections::HashMap;
// use std::collections::HashSet;
// use std::fmt;

#[derive(Debug, Copy, Clone)]
enum SeatState {
    Empty,
    Occupied,
}

use SeatState::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

impl Location {
    fn new() -> Self {
        Location { x: 0, y: 0 }
    }
}

fn main() {
    aoc::preamble();
    // part1();
    part2();
}

fn stats_at_end() -> (i64, i64) {
    let state = parse_lines(file::lines());
    let mut count = 1;
    let mut prev_stats = seat_stats(&state);
    let mut state = next_map(&state);
    let mut current_stats = seat_stats(&state);
    while prev_stats != current_stats {
        prev_stats = current_stats;
        state = next_map(&state);
        current_stats = seat_stats(&state);
        println!("Iteration {} stats: {:?}", count, current_stats);
        count += 1;
    }
    println!("End stats {:?}", current_stats);
    current_stats
}

fn part1() {
    //    let result = "None Yet";

    let stats = stats_at_end();
    let result = stats.0;
    aoc::print_solution1(format!("{:?}", result).as_str());
}

fn generate_seat_map(seats: &HashMap<Location, SeatState>) -> HashMap<Location, Vec<Location>> {
    let mut max_x = 0i64;
    let mut max_y = 0i64;
    for (loc, _) in seats {
        max_x = max_x.max(loc.x);
        max_y = max_y.max(loc.y);
    }

    let mut result = HashMap::new();
    for (loc, _) in seats {
        let mut relevants: Vec<Location> = Vec::new();
        for x in -1i64..=1 {
            for y in -1i64..=1 {
                if (x, y) != (0, 0) {
                    // search for relevance
                    let movement = Location { x, y };
                    let mut test_loc = *loc;
                    loop {
                        test_loc = test_loc + movement;
                        if let Some(_) = seats.get(&test_loc) {
                            relevants.push(test_loc);
                            break;
                        } else if (test_loc.x < 0 || test_loc.x > max_x)
                            || (test_loc.y < 0 || test_loc.y > max_y)
                        {
                            break;
                        }
                    }
                }
            }
        }

        result.insert(*loc, relevants);
    }
    result
}

fn next_map_p2(
    seats: &HashMap<Location, SeatState>,
    relevants: &HashMap<Location, Vec<Location>>,
) -> HashMap<Location, SeatState> {
    let mut result = HashMap::new();
    for (loc, state) in seats {
        let seat_count = relevants[loc].iter().fold(0, |acc, test_loc| {
            if let Some(Occupied) = seats.get(test_loc) {
                acc + 1
            } else {
                acc
            }
        });
        match seat_count {
            0 => result.insert(*loc, Occupied),
            5..=9 => result.insert(*loc, Empty),
            _ => result.insert(*loc, *state),
        };
    }
    result
}

fn part2() {
    // let result = "None Yet";

    let stats = stats_at_end_p2();
    let result = stats.0;
    aoc::print_solution2(format!("{:?} ", result).as_str());
}

fn stats_at_end_p2() -> (i64, i64) {
    let state = parse_lines(file::lines());
    let seat_map = generate_seat_map(&state);
    let mut count = 1;
    let mut prev_stats = seat_stats(&state);
    let mut state = next_map_p2(&state, &seat_map);
    let mut current_stats = seat_stats(&state);
    while prev_stats != current_stats {
        prev_stats = current_stats;
        state = next_map_p2(&state, &seat_map);
        current_stats = seat_stats(&state);
        println!("Iteration {} stats: {:?}", count, current_stats);
        count += 1;
    }
    println!("End stats {:?}", current_stats);
    current_stats
}

fn parse_lines(lines: impl Iterator<Item = String>) -> HashMap<Location, SeatState> {
    let mut result = HashMap::new();
    for (row, line) in lines.enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == 'L' {
                result.insert(
                    Location {
                        x: row as i64,
                        y: col as i64,
                    },
                    Empty,
                );
            }
        }
    }
    result
}

fn seat_stats(seats: &HashMap<Location, SeatState>) -> (i64, i64) {
    let mut empty_count = 0;
    let mut occupied_count = 0;
    for (_loc, state) in seats {
        match state {
            Occupied => occupied_count += 1,
            Empty => empty_count += 1,
        }
    }
    (occupied_count, empty_count)
}

fn seat_count_at(seats: &HashMap<Location, SeatState>, loc: &Location) -> i64 {
    let mut result = 0;
    for row in (loc.x - 1)..=(loc.x + 1) {
        for col in (loc.y - 1)..=(loc.y + 1) {
            let test_loc = Location { x: row, y: col };
            if test_loc != *loc {
                if let Some(Occupied) = seats.get(&test_loc) {
                    result += 1;
                }
            }
        }
    }
    result
}

fn next_map(seats: &HashMap<Location, SeatState>) -> HashMap<Location, SeatState> {
    let mut result = HashMap::new();
    for (loc, state) in seats {
        match seat_count_at(seats, loc) {
            0 => result.insert(*loc, Occupied),
            4..=9 => result.insert(*loc, Empty),
            _ => result.insert(*loc, *state),
        };
    }
    result
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
        let source = indoc! {"L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL"};
        source.split("\n").map(|l| l.to_string())
    }

    #[test]
    fn test_part1() {
        let lines: Vec<String> = test_lines().collect();
        println!("input: {:?}", lines);
        let state = parse_lines(test_lines());
        println!("{:?}", state);

        let mut count = 1;
        let mut state = next_map(&state);
        let mut prev_stats = (0i64, 0i64);
        let mut current_stats = seat_stats(&state);
        while prev_stats != current_stats {
            prev_stats = current_stats;
            state = next_map(&state);
            current_stats = seat_stats(&state);
            println!("Iteration {} stats: {:?}", count, current_stats);
            count += 1;
        }
        println!("End stats {:?}", current_stats);

        assert_eq!(current_stats.0, 37);
    }
}
