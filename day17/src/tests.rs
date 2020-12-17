// note
// this outputs standard io while running tests
// cargo test -- --nocapture

// also cool (needs cargo-watch installed)
// cargo watch -x "test -- --nocapture"

use std::collections::HashMap;

use indoc::indoc;

#[cfg(test)]
use super::*;

fn test_lines() -> impl Iterator<Item = String> {
    let source = indoc! {".#.
    ..#
    ###"};
    source.split("\n").map(|l| l.to_string())
}

fn solve_part1(lines: impl Iterator<Item = String>) -> i64 {
    for line in lines {
        println!("{}", line);
    }
    12
}

#[test]
fn test_part1() {
    assert_eq!(solve_part1(test_lines()), 112);
}

// #[test]
// fn test_part2() {
//     assert_eq!(solve_part2(test_lines_p2()), 1);
// }
