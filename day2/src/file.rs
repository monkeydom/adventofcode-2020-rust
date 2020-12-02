use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn lines() -> impl Iterator<Item = String> {
    let file = File::open("input.txt").expect("Expected file to live at ./input.txt");
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap())
}
