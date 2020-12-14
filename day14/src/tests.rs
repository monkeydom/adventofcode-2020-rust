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

fn parse_mask(mask: &str) -> (u64, u64) {
    let mut set_mask = 0u64;
    let mut clear_mask = 0u64;

    for (loc, c) in mask.chars().enumerate() {
        match c {
            '1' => set_mask |= 1u64 << (35 - loc),
            '0' => clear_mask |= 1u64 << (35 - loc),
            _ => (),
        }
    }

    (set_mask, clear_mask)
}

fn process_lines(lines: impl Iterator<Item = String>) -> HashMap<usize, u64> {
    let mut result = HashMap::new();

    let mut set_mask = 0u64;
    let mut clear_mask = 0u64;

    for line in lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        match parts[0] {
            "mask" => {
                let pm = parse_mask(parts[1]);
                set_mask = pm.0;
                clear_mask = pm.1;
            }

            _ => {
                let loc: usize = parts[0][4..(parts[0].len()-1)].parse().unwrap();
                let value: u64 = parts[1].parse().unwrap();
                let value = value | set_mask;
                let value = value & !clear_mask;
                result.insert(loc, value);
            }
        }
    }

    result
}

fn sum_memory(mem: &HashMap<usize, u64>) -> u64 {
    mem.iter().fold(0u64, |acc, (_,v)| acc + *v)
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
fn test_parse_mask() {
    assert_eq!(parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"), (1<<6, 1<<1));
}