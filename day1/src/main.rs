use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    println!("AoC 2020 Day 1:");

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let input_numbers = reader.lines().map(|l| {
        l.unwrap()
            .parse::<i64>()
            .expect("Expected parseable number in each line")
    });

    for number in input_numbers {
        print!("{} ", number);
    }

    println!("");
    println!("Done");

    Ok(())
}
