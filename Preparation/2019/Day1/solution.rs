use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn fuel_consumption(mass : i64) -> i64 {
    std::cmp::max(mass / 3 - 2, 0)
}

fn main() -> io::Result<()> {
    println!("Hello World!");

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    
    let mut totalFuel : i64 = 0;
    let mut totalFuelPart2 : i64 = 0;

    for number in reader.lines().map(|l| l.unwrap().parse::<i64>()) {
        let mass = number.unwrap();
        let mut fuel = fuel_consumption(mass);
        totalFuel += fuel;
        print!("{} | {}", mass, fuel);
        while fuel > 0 {
            totalFuelPart2 += fuel;
            fuel = fuel_consumption(fuel);
            print!(" {}", fuel);
        }
        println!("");
    }

    println!("Solution to Part1: {}\nSolution to Part2: {}", totalFuel, totalFuelPart2);

    Ok(())
}