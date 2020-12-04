mod aoc;
mod file;
use std::collections::HashSet;

fn main() {
    aoc::preamble();

    part1();
    part2();
}

fn required_keys() -> HashSet<&'static str> {
    let array = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; // , "cid"]; cid optional
    let mut result = HashSet::<&str>::new();
    for s in array.iter() {
        result.insert(s);
    }
    result
}

fn part1() {
    let mut viable_passports = 0;
    let mut outstanding_keys = required_keys();

    let check = |keys: &HashSet<&str>, count: &mut i32| {
        if keys.is_empty() {
            *count += 1;
            println!("🎫 viable! [{}]", count);
        }
    };

    for line in file::lines() {
        if line == "" {
            check(&outstanding_keys, &mut viable_passports);
            outstanding_keys = required_keys();
            println!("");
        }
        for element in line.split_whitespace() {
            let keyval: Vec<&str> = element.splitn(2, ":").collect();
            aoc::print_key_value(keyval[0], keyval[1]);
            outstanding_keys.remove(keyval[0]);
        }
    }
    check(&outstanding_keys, &mut viable_passports);

    aoc::print_solution1(format!("{} viable 🎫", viable_passports).as_str());
}

fn part2() {
    aoc::print_solution2("Not Yet");
}
