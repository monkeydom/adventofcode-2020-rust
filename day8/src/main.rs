#[allow(dead_code)]
mod aoc;
#[allow(dead_code)]
mod file;
// use itertools::Itertools;
// use ansi_term::Colour::Red;
// use std::{collections::HashSet, ops::RangeBounds};
// use std::collections::HashMap;
use std::collections::HashSet;
// use std::fmt;

fn main() {
    aoc::preamble();
    part1();
    part2();
}

#[derive(Debug)]
enum Instruction {
    Acc(i64),
    Jmp(isize),
    Nop,
}

impl Instruction {}

fn part1() {
    let result = "None Yet";

    let mut program: Vec<Instruction> = vec![];
    let mut visited: HashSet<usize> = HashSet::new();

    for ins in file::lines() {
        let pair: Vec<&str> = ins.split(" ").collect();
        let n: i64 = pair[1].parse().expect("Needs to be a parseable number");
        let i = match pair[0] {
            "jmp" => Instruction::Jmp(n as isize),
            "acc" => Instruction::Acc(n),
            "nop" => Instruction::Nop,
            _ => panic!("Unknown instruction found! {}", ins),
        };

        program.push(i);
    }

    let mut pc: usize = 0;
    let mut acc: i64 = 0;
    let mut stepcount: i64 = 0;

    while !visited.contains(&pc) {
        visited.insert(pc);
        let ins = &program[pc];

        println!("{}: {:?}", stepcount, (pc, acc, ins));

        pc = (pc as isize
            + match ins {
                Instruction::Jmp(n) => *n,
                Instruction::Acc(n) => {
                    acc += n;
                    1
                }
                Instruction::Nop => 1,
            }) as usize;
        stepcount += 1;
    }

    let result = (pc, acc, &program[pc]);

    aoc::print_solution1(format!("{:?}", result).as_str());
}

fn part2() {
    let result = "None Yet";

    aoc::print_solution2(format!("{:?} ", result).as_str());
}
