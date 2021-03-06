use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    println!("AoC 2020 Day 1:");

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut input_numbers: Vec<i64> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .parse::<i64>()
                .expect("Expected parseable number in each line")
        })
        .collect();

    input_numbers.sort();

    let mut reversed = input_numbers.clone();
    reversed.reverse();

    let threshold = 2020;
    let mut a = -1;
    let mut b = -1;

    'outer: for number in &input_numbers {
        print!("{} ", *number);
        for other in &reversed {
            let sum = *other + *number;
            if sum < threshold {
                println!("Sum to low {} + {} = {}", *other, *number, sum);
                break;
            } else if sum == threshold {
                println!("Sum good {} + {} = {}", *other, *number, sum);
                a = *number;
                b = *other;
                break 'outer;
            }
        }
    }

    println!("");
    println!("Solution Part 1: {} * {} = {}", a, b, a * b);
    println!("");

    let mut c = -1;

    'outer2: for biggest in reversed {
        let mut ascending = input_numbers.clone();
        while ascending.len() > 0 {
            let second = ascending.remove(0);
            for third in &ascending {
                let sum = biggest + second + *third;

                if sum > threshold {
                    break;
                } else if sum == threshold {
                    a = second;
                    b = *third;
                    c = biggest;
                    println!("Sum good {} + {} + {} = {}", a, b, c, sum);
                    break 'outer2;
                }
            }
        }
    }

    println!("Solution Part 2: {} * {} * {} = {}", a, b, c, a * b * c);

    println!("Done");

    Ok(())
}
