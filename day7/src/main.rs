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
    // part1();
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

    let contents: Vec<BagContents> = file::lines().map(|l| parse_rule(l)).collect();

    let mut containment_count = HashMap::new();

    let gold_bag = BagType::from_strings(&["shiny", "gold"]);

    let mut cc = &mut containment_count;
    let mut update = |mcc: &mut HashMap<BagType, i64>, bt: &BagType, count: i64| {
        mcc.insert(bt.clone(), mcc.get(bt).unwrap_or(&0) + count);
    };

    for bc in contents.iter() {
        if bc.contents.len() == 0 {
            println!("LEN 0 ! {:?}", bc);
            update(&mut cc, &bc.bt, 0);
        }
    }

    let mut iteration = 1;
    loop {
        let mut unknown_count = 0;
        let mut updated_some = false;
        for bc in contents.iter() {
            if let None = cc.get(&bc.bt) {
                if bc.contents.iter().all(|bt| cc.get(&bt).is_some()) {
                    updated_some = true;
                    let value = bc
                        .contents
                        .iter()
                        .fold(0, |acc, v| acc + cc.get(v).unwrap() + 1);
                    println!("{:?} -> updating with value  {}", &bc.bt, value);
                    update(cc, &bc.bt, value);
                } else {
                    println!("unknown {:?}", &bc.bt);
                    unknown_count += 1;
                }
            }
        }
        if !updated_some {
            break;
        }
        println!(
            "====== {} iteration [uk: {}] ======",
            iteration, unknown_count
        );
        iteration += 1;
    }

    //    println!("{:?}", containment_count);

    let result = containment_count.get(&gold_bag);

    aoc::print_solution2(format!("{:?} ", result).as_str());
}
