#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[allow(dead_code)]
mod aoc;
#[allow(dead_code)]
mod file;

mod tests;

// use itertools::Itertools;
// use ansi_term::Colour::Red;
// use std::{collections::HashSet, ops::RangeBounds};
// use std::collections::HashMap;
use std::collections::HashSet;
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
    //    let result = "None Yet";
    let result = solve_part2(file::lines());
    aoc::print_solution2(format!("{:?} ", result).as_str());
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Point(i64, i64, i64);

fn adjacent_points(p: &Point) -> impl Iterator<Item = Point> {
    vec![
        Point(p.0 + 1, p.1 + 1, p.2 + 1),
        Point(p.0 + 1, p.1 + 1, p.2),
        Point(p.0, p.1 + 1, p.2 + 1),
        Point(p.0 + 1, p.1, p.2 + 1),
        Point(p.0 + 1, p.1, p.2),
        Point(p.0, p.1 + 1, p.2),
        Point(p.0, p.1, p.2 + 1),
        Point(p.0 - 1, p.1 - 1, p.2 - 1),
        Point(p.0 - 1, p.1 - 1, p.2),
        Point(p.0, p.1 - 1, p.2 - 1),
        Point(p.0 - 1, p.1, p.2 - 1),
        Point(p.0 - 1, p.1, p.2),
        Point(p.0, p.1 - 1, p.2),
        Point(p.0, p.1, p.2 - 1),
        Point(p.0 - 1, p.1, p.2 + 1),
        Point(p.0 + 1, p.1, p.2 - 1),
        Point(p.0, p.1 - 1, p.2 + 1),
        Point(p.0, p.1 + 1, p.2 - 1),
        Point(p.0 - 1, p.1 + 1, p.2),
        Point(p.0 + 1, p.1 - 1, p.2),
        Point(p.0 - 1, p.1 - 1, p.2 + 1),
        Point(p.0 - 1, p.1 + 1, p.2 - 1),
        Point(p.0 + 1, p.1 - 1, p.2 - 1),
        Point(p.0 - 1, p.1 + 1, p.2 + 1),
        Point(p.0 + 1, p.1 - 1, p.2 + 1),
        Point(p.0 + 1, p.1 + 1, p.2 - 1),
    ]
    .into_iter()
}

fn perform_step(state: HashSet<Point>) -> HashSet<Point> {
    let mut result = HashSet::new();
    let mut adjacents: HashSet<Point> = HashSet::new();

    for loc in &state {
        let mut other_count = 0;
        for p in adjacent_points(loc) {
            if state.contains(&p) {
                other_count += 1;
            }
            adjacents.insert(p);
        }
        // println!("{:?} = {:?}", loc, other_count);
        if other_count >= 2 && other_count <= 3 {
            result.insert(*loc);
        }
    }

    for loc in &adjacents {
        let mut other_count = 0;
        for p in adjacent_points(loc) {
            if state.contains(&p) {
                other_count += 1;
            }
        }
        if other_count == 3 {
            result.insert(*loc);
        }
        // if loc.2 == 0 {
        //     println!("{:?} = {:?}", loc, other_count);
        // }
    }

    // println!("New state: {:?}", &result);

    result
}

fn parse_to_state(lines: impl Iterator<Item = String>) -> HashSet<Point> {
    let mut result = HashSet::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                result.insert(Point(x as i64, y as i64, 0));
            }
        }
        println!("{}: {}", y, line);
    }

    result
}

fn solve_part1(lines: impl Iterator<Item = String>) -> i64 {
    let mut state = parse_to_state(lines);
    println!("{:?}", state);
    state = perform_step(state);

    // for y in 0..4 {
    //     for x in 0..3 {
    //         print!(
    //             "{}",
    //             (if state.contains(&Point(x, y, 0)) {
    //                 "#"
    //             } else {
    //                 "."
    //             })
    //         )
    //     }
    //     println!("");
    // }

    // println!("{:?}", state);
    state = perform_step(state);
    state = perform_step(state);
    state = perform_step(state);
    state = perform_step(state);
    state = perform_step(state);
    state.len() as i64
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Point4(i64, i64, i64, i64);

fn solve_part2(lines: impl Iterator<Item = String>) -> i64 {
    let mut state: HashSet<Point4> = parse_to_state(lines)
        .iter()
        .map(|p| Point4(p.0, p.1, p.2, 0))
        .collect();
    // println!("{:?}", state);
    state = perform_step4(state);

    // for y in 0..4 {
    //     for x in 0..3 {
    //         print!(
    //             "{}",
    //             (if state.contains(&Point4(x, y, 0, 0)) {
    //                 "#"
    //             } else {
    //                 "."
    //             })
    //         )
    //     }
    //     println!("");
    // }

    println!("1");
    // println!("{:?}", state);
    state = perform_step4(state);
    println!("2");
    state = perform_step4(state);
    println!("3");
    state = perform_step4(state);
    println!("4");
    state = perform_step4(state);
    println!("5");
    state = perform_step4(state);
    println!("6");
    state.len() as i64
}

fn perform_step4(state: HashSet<Point4>) -> HashSet<Point4> {
    let mut result = HashSet::new();
    let mut adjacents: HashSet<Point4> = HashSet::new();

    for loc in &state {
        let mut other_count = 0;
        for p in adjacent_points4(loc) {
            if state.contains(&p) {
                other_count += 1;
            }
            adjacents.insert(p);
        }
        // println!("{:?} = {:?}", loc, other_count);
        if other_count >= 2 && other_count <= 3 {
            result.insert(*loc);
        }
    }

    for loc in &adjacents {
        let mut other_count = 0;
        for p in adjacent_points4(loc) {
            if state.contains(&p) {
                other_count += 1;
            }
        }
        if other_count == 3 {
            result.insert(*loc);
        }
        // if loc.2 == 0 && loc.3 == 0 {
        //     println!("{:?} = {:?}", loc, other_count);
        // }
    }

    // println!("New state: {:?}", &result);

    result
}

fn adjacent_points4(p: &Point4) -> impl Iterator<Item = Point4> {
    let mut v: Vec<Point4> = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    let c = Point4(x + p.0, y + p.1, z + p.2, w + p.3);
                    if c != *p {
                        v.push(c);
                    }
                }
            }
        }
    }

    v.into_iter()
}
