mod aoc;
mod file;
// use itertools::Itertools;
// use ansi_term::Colour::Red;
// use std::{collections::HashSet, ops::RangeBounds};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

fn main() {
    aoc::preamble();
    part1();
    part2();
}

#[derive(Debug)]
struct BagContents {
    bt: BagType,
    contents: Vec<BagType>,
}

impl BagContents {
    fn from_strings(strings: &[&str]) -> Self {
        let bt = BagType::from_strings(&strings[..2]);
        let mut contents: Vec<BagType> = vec![];
        for index in (4..strings.len()).step_by(4) {
            if let Ok(count) = strings[index].parse() {
                let bagtype = BagType::from_strings(&strings[index + 1..index + 3]);
                contents.extend((0..count).map(|_| bagtype.clone()));
            }
        }
        BagContents { bt, contents }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct BagType {
    attribute: String,
    color: String,
}

impl fmt::Display for BagType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "[{} {}]", self.attribute, self.color)
    }
}

impl BagType {
    fn from_strings(strings: &[&str]) -> Self {
        BagType {
            attribute: strings[0].to_string(),
            color: strings[1].to_string(),
        }
    }
}

fn parse_rule(line: String) -> BagContents {
    let tokens: Vec<&str> = line.split(" ").collect();
    // let bag = BagType::from_strings(&tokens[..2]);
    let bag_contents = BagContents::from_strings(&tokens);
    println!("{}\n{:?}\n", line, &bag_contents);
    bag_contents
}

fn part1() {
//    let result = "None Yet";

    let contents: Vec<BagContents> = file::lines().map(|l| parse_rule(l)).collect();

    let innermost_bag = BagType::from_strings(&["shiny", "gold"]);

    let mut hash_set: HashSet<BagType> = HashSet::new();
    hash_set.insert(innermost_bag);
    loop {
        let mut collect_set: HashSet<BagType> = HashSet::new();
        for bc in contents.iter() {
            for inner in bc.contents.iter() {
                if hash_set.contains(inner) {
                    collect_set.insert(bc.bt.clone());
                }
            }
        }

        for bt in &hash_set {
            collect_set.insert(bt.clone());
        }

        if collect_set.len() > hash_set.len() {
            hash_set = collect_set;
        } else {
            break;
        }

    }

    let result = hash_set.len() - 1;

    aoc::print_solution1(format!("{}", result).as_str());
}

fn part2() {
    let result = "None Yet";
    aoc::print_solution2(format!("{} ", result).as_str());
}
