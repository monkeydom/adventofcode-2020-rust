use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn lines() -> impl Iterator<Item = String> {
    let file = File::open("input.txt").expect("Expected file to live at ./input.txt");
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap())
}

pub fn sections() -> impl Iterator<Item = impl Iterator<Item = String>> {
    let mut intermediate: Vec<Vec<String>> = vec![];
    let mut current: Vec<String> = vec![];
    for line in lines() {
        if line != "" {
            current.push(line);
        } else {
            if current.len() > 0 {
                intermediate.push(current);
            }
            current = vec![]
        }
    }
    if current.len() > 0 {
        intermediate.push(current);
    }

    intermediate.into_iter().map(|v| v.into_iter())
}
