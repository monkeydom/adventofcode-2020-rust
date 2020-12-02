mod file;
mod format;

fn main() {
    println!("2020 Day 2");
    println!("==========");

    for (i, line) in file::lines().enumerate() {
        println!("{} [{}]: {}", i, format::bool(validate(&line)), line);
    }
}

fn validate(line: &String) -> bool {
    let elements: Vec<&str> = (*line).split_whitespace().collect();
    let pwd = elements[2];
    let range = parse_range(elements[0]);
    let c = elements[1]
        .chars()
        .next()
        .expect("Expected a character at position 2");

    print!("<{:?}> ", (range, c, pwd));
    true
}

fn parse_range(r: &str) -> std::ops::RangeInclusive<usize> {
    let mut elements = r.split("-").map(|n| {
        n.parse::<usize>()
            .expect("Needed to be a number in the range spec")
    });
    std::ops::RangeInclusive::new(elements.next().unwrap(), elements.next().unwrap())
}
