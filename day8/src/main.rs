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

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(i64),
    Jmp(isize),
    Nop(isize),
}
impl Instruction {}

fn run(program: &Vec<Instruction>) -> (i64, usize, i64, Instruction) {
    let mut pc: usize = 0;
    let mut acc: i64 = 0;
    let mut stepcount: i64 = 0;
    let mut visited: HashSet<usize> = HashSet::new();

    while !visited.contains(&pc) {
        if pc >= program.len() {
            let result = acc;
            aoc::print_solution2(format!("{:?} ", result).as_str());
            panic!("Solution {:?}", (stepcount, pc, acc, program[pc - 1]));
        }

        visited.insert(pc);
        let ins = &program[pc];

        // println!("{}: {:?}", stepcount, (pc, acc, ins));

        pc = (pc as isize
            + match ins {
                Instruction::Jmp(n) => *n,
                Instruction::Acc(n) => {
                    acc += n;
                    1
                }
                Instruction::Nop(_) => 1,
            }) as usize;
        stepcount += 1;
    }

    (stepcount, pc, acc, program[pc])
}

fn read_program() -> Vec<Instruction> {
    let mut program: Vec<Instruction> = vec![];
    for ins in file::lines() {
        let pair: Vec<&str> = ins.split(" ").collect();
        let n: i64 = pair[1].parse().expect("Needs to be a parseable number");
        let i = match pair[0] {
            "jmp" => Instruction::Jmp(n as isize),
            "acc" => Instruction::Acc(n),
            "nop" => Instruction::Nop(n as isize),
            _ => panic!("Unknown instruction found! {}", ins),
        };

        program.push(i);
    }
    program
}

fn part1() {
    let result = "None Yet";

    let program = read_program();
    let result = run(&program);

    aoc::print_solution1(format!("{:?}", result).as_str());
}

fn part2() {
    let result = "None Yet";

    let mut program = read_program();
    let mut index = 0;

    loop {
        let ins = &program[index].clone();
        match ins {
            Instruction::Nop(n) => {
                program[index] = Instruction::Jmp(*n);
                println!("Switch at {} : {:?}", index, run(&program));
                program[index] = *ins;
            }
            Instruction::Jmp(n) => {
                program[index] = Instruction::Nop(*n);
                println!("Switch at {} : {:?}", index, run(&program));
                program[index] = *ins;
            }

            _ => (),
        }
        index += 1;
        if index >= program.len() {
            break;
        }
    }

    aoc::print_solution2(format!("{:?} ", result).as_str());
}


#[cfg(test)]
mod tests {
    #[test]
    fn simple_test_1() {
        assert_eq!(2+2, 4);
    }
}
