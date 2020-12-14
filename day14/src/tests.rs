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
    let source = indoc! {"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
							mem[8] = 11
							mem[7] = 101
							mem[8] = 0"};
    source.split("\n").map(|l| l.to_string())
}

fn test_lines2() -> impl Iterator<Item = String> {
    let source = indoc! {"mask = 000000000000000000000000000000X1001X
    mem[42] = 100
    mask = 00000000000000000000000000000000X0XX
    mem[26] = 1"};
    source.split("\n").map(|l| l.to_string())
}

#[test]
fn test_part1() {
    let lines: Vec<String> = test_lines().collect();
    println!("{:?}", lines);
    let mem = process_lines(test_lines());
    println!("{:?}", &mem);
    assert_eq!(sum_memory(&mem), 165);
}

#[test]
fn test_part2() {
    let lines: Vec<String> = test_lines2().collect();
    println!("{:?}", lines);
    let mem = process_lines_p2(test_lines2());
    println!("{:?}", &mem);
    assert_eq!(sum_memory(&mem), 208);
}

#[test]
fn test_parse_mask() {
    let (a, b, _) = parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
    assert_eq!((a, b), (1 << 6, 1 << 1));
}
