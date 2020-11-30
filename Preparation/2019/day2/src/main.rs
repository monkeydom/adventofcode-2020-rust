use itertools::Itertools;
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

    let mut program: Vec<i64> = line
        .split(",")
        .map(|n| n.parse::<i64>().expect("Expected Integer String"))
        .collect();

    let mut pc: usize = 0;

    // adjust program as per instruction
    restore(&mut program);

    while program[pc] != 99 {
        execute(&mut program, &mut pc);
    }

    println!("End encountered at position {}", pc);

    let result: String = program
        .iter()
        .map(|&n| n.to_string())
        .intersperse(",".to_string())
        .collect();
    println!("End state: {}", result);

    Ok(())
}

fn execute(prog: &mut Vec<i64>, pc: &mut usize) {
    println!("{}: {}", *pc, prog[*pc]);
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

fn restore(prog: &mut Vec<i64>) {
    prog[1] = 12;
    prog[2] = 2;
}
