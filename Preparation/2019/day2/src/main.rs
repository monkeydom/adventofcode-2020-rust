// use itertools::Itertools;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    println!("AoC 2019 Day2:");
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let inputline = reader
        .lines()
        .next()
        .unwrap()
        .expect("Expected line of integers separated by comma");

    let line = inputline;
    // let line = "1,0,0,0,99";
    // let line = "2,3,0,3,99";
    // let line = "2,4,4,5,99,0";
    // let line = "1,1,1,4,99,5,6,0,99";

    if progresult("1,0,0,0,99", 0, 0) != 2 {
        return Err(io::Error::new(io::ErrorKind::Other, "something went wrong"));
    }

    println!("Result of Part 1 is: {}", progresult(&line, 12, 2));

    let desired_output: i64 = 19690720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            if progresult(&line, noun, verb) == desired_output {
                println!(
                    "Result of part 2 is: Noun {} Verb {} - final result {}",
                    noun,
                    verb,
                    noun * 100 + verb
                );
                return Ok(());
            }
        }
    }

    Ok(())
}

fn execute(prog: &mut Vec<i64>, pc: &mut usize) {
    let store = prog[*pc + 3] as usize;
    let a = prog[prog[*pc + 1] as usize];
    let b = prog[prog[*pc + 2] as usize];
    if prog[*pc] == 1 {
        prog[store] = a + b;
    } else {
        prog[store] = a * b;
    }
    *pc += 4;
}

fn progresult(prog_string: &str, noun: i64, verb: i64) -> i64 {
    let mut program: Vec<i64> = prog_string
        .split(",")
        .map(|n| n.parse::<i64>().expect("Expected Integer String"))
        .collect();

    let mut pc: usize = 0;

    // adjust program as per instruction
    program[1] = noun;
    program[2] = verb;

    while program[pc] != 99 {
        execute(&mut program, &mut pc);
    }
    return program[0];
}
