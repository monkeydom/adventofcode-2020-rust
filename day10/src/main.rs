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
    // part2();
}

fn part1() {
    // let result = "None Yet";
    let mut jolts = parse_numbers(file::lines());
    jolts.sort();
    println!("!{:?}", jolts);
    let (one, _, three) = jolt_distances(&jolts);

    let result = one * three;
    aoc::print_solution1(format!("{:?}", result).as_str());

    let result = count_possibilities(&jolts);
    
    aoc::print_solution2(format!("{:?} ", result).as_str());

}

// fn part2() {
//     let result = "None Yet";
//     aoc::print_solution2(format!("{:?} ", result).as_str());
// }

fn jolt_distances(jolts: &Vec<i64>) -> (i64, i64, i64) {
    let (one, two, three, _) = jolts.iter().fold((1, 0, 1, -10), |acc, v| match v - acc.3 {
        1 => (acc.0 + 1, acc.1, acc.2, *v),
        2 => (acc.0, acc.1 + 1, acc.2, *v),
        3 => (acc.0, acc.1, acc.2 + 1, *v),
        _ => (acc.0, acc.1, acc.2, *v),
    });

    (one, two, three)
}

fn count_possibilities_r(jolt: i64, jolts: &[i64]) -> usize {
    //println!("{}", jolt);
    let mut result = 0;
    if jolts.len() <= 1 { return 1; }
    for index in 0..(jolts.len().min(3)) {
        let v = jolts[index];
            if v - jolt <= 3 {
                result += count_possibilities_r(v, &jolts[(index+1)..]);
            }
        
    }
//    println!("Returning {} from {:?}", result, (jolt, &jolts));   
    result 
}

fn count_possibilities(jolts: &Vec<i64>) -> usize {
    count_possibilities_r(0, &jolts[..])
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
        let source = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        source.split("\n").map(|l| l.to_string())
    }

    fn test_lines2() -> impl Iterator<Item = String> {
        "16
10
15
5
1
11
7
19
6
12
4"
        .split("\n")
        .map(|l| l.to_string())
    }

    #[test]
    fn test_short_testset() {
        let mut jolts = parse_numbers(test_lines2());
        jolts.sort();
        println!("!{:?}", jolts);
        let (one, _, three) = jolt_distances(&jolts);
        assert_eq!((one, three), (7, 5));
    }

    #[test]
    fn test_longer_testset() {
        let mut jolts = parse_numbers(test_lines());
        jolts.sort();
        println!("!{:?}", jolts);
        let (one, _, three) = jolt_distances(&jolts);
        assert_eq!((one, three), (22, 10));
    }


    #[test]
    fn test_short_testset_p2() {
        let mut jolts = parse_numbers(test_lines2());
        jolts.sort();
        println!("!{:?}", jolts);
        let result = count_possibilities(&jolts);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_longer_testset_p2() {
        let mut jolts = parse_numbers(test_lines());
        jolts.sort();
        println!("!{:?}", jolts);
        let result = count_possibilities(&jolts);
        assert_eq!(result, 19208);
    }
}
