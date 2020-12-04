mod aoc;
mod file;
use ansi_term::Colour::Red;
use std::{collections::HashSet, ops::RangeBounds};

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

fn is_year_in_range<T>(r: T, value: &str) -> bool
where
    T: RangeBounds<i32>,
{
    value.len() == 4 && {
        if let Ok(v) = value.parse::<i32>() {
            r.contains(&v)
        } else {
            false
        }
    }
}

fn is_valid_eyecolor(v: &str) -> bool {
    match v {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn is_valid_number(digits: usize, v: &str) -> bool {
    v.len() == digits && v.chars().all(char::is_numeric)
}

fn is_valid_haircolor(v: &str) -> bool {
    let mut check = v.chars();
    if Some('#') == check.next() {
        check.all(|c| c.is_ascii_hexdigit())
    } else {
        false
    }
}

fn is_valid_height(v: &str) -> bool {
    let number_part: String = v.chars().take_while(|c| c.is_numeric()).collect();
    let unit = &v[number_part.len()..];

    if let Ok(n) = number_part.parse::<i32>() {
        match unit {
            "cm" => (150..=193).contains(&n),
            "in" => (59..=76).contains(&n),
            _ => false,
        }
    } else {
        false
    }
}

fn validate_and_remove(keys: &mut HashSet<&str>, key: &str, value: &str) -> bool {
    let result = match key {
        "byr" => is_year_in_range(1920..=2002, value),
        "iyr" => is_year_in_range(2010..=2020, value),
        "eyr" => is_year_in_range(2020..=2030, value),
        "ecl" => is_valid_eyecolor(value),
        "pid" => is_valid_number(9, value),
        "hcl" => is_valid_haircolor(value),
        "hgt" => is_valid_height(value),
        _ => true,
    };

    if result {
        keys.remove(key);
        true
    } else {
        println!(
            "{}",
            Red.bold()
                .paint(format!("{} is invalid for {}!", value, key))
        );
        false
    }
}

fn part2() {
    let mut viable_passports = 0;
    let mut still_valid = true;
    let mut outstanding_keys = required_keys();

    let check = |keys: &HashSet<&str>, count: &mut i32| {
        if keys.is_empty() {
            *count += 1;
            println!("🎫 viable! [{}]", count);
        }
    };

    'lineloop: for line in file::lines() {
        if line == "" {
            check(&outstanding_keys, &mut viable_passports);
            outstanding_keys = required_keys();
            still_valid = true;
            println!("");
        } else if still_valid {
            for element in line.split_whitespace() {
                let keyval: Vec<&str> = element.splitn(2, ":").collect();
                aoc::print_key_value(keyval[0], keyval[1]);
                if !validate_and_remove(&mut outstanding_keys, keyval[0], keyval[1]) {
                    still_valid = false;
                    continue 'lineloop;
                }
            }
        }
    }
    check(&outstanding_keys, &mut viable_passports);

    aoc::print_solution2(format!("{} viable and validated 🎫", viable_passports).as_str());

    assert_eq!(is_valid_height("63in"), true);
}
