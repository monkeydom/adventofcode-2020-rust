use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    println!("Hello World!");

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    
    let mut totalFuel : i64 = 0;

    for number in reader.lines().map(|l| l.unwrap().parse::<i64>()) {
        let mass = number.unwrap();
        let fuel = mass / 3 - 2;
        totalFuel += fuel;
        println!("{} {} -> total fuel {}", mass, fuel, totalFuel);

    }

    Ok(())
}