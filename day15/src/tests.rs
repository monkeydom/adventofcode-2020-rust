// note
// this outputs standard io while running tests
// cargo test -- --nocapture

// also cool (needs cargo-watch installed)
// cargo watch -x "test -- --nocapture"

use std::collections::HashMap;

use indoc::indoc;

#[cfg(test)]
use super::*;

#[test]
fn test_game_p1() {
    assert_eq!(nth_number(vec![0, 3, 6], 10), 0);
    assert_eq!(nth_number(vec![0, 3, 6], 2020), 436);
    assert_eq!(nth_number(vec![1, 3, 2], 2020), 1);
    assert_eq!(nth_number(vec![2, 1, 3], 2020), 10);
    assert_eq!(nth_number(vec![1, 2, 3], 2020), 27);
}

#[test]
fn test_game_p2() {
    assert_eq!(nth_number(vec![0, 3, 6], 30_000_000), 175594);
    assert_eq!(nth_number(vec![1, 3, 2], 30_000_000), 2578);
    assert_eq!(nth_number(vec![2, 1, 3], 30_000_000), 3544142);
    assert_eq!(nth_number(vec![1, 2, 3], 30_000_000), 261214);
}
