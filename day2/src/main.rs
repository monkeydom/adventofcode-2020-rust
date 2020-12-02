mod file;
mod format;

fn main() {
    println!("2020 Day 2");
    println!("==========");

    part1();

    part2();
}

fn part1() {
    let mut correct_count = 0;
    let mut invalid_count = 0;

    for (_i, line) in file::lines().enumerate() {
        let was_correct = validate(&line);
        // println!("{} [{}]: {}", i, format::bool(was_correct), line);
        if was_correct {
            correct_count += 1;
        } else {
            invalid_count += 1;
        }
    }

    println!("\nPart 1:");
    println!(
        "{}: {}/{total} | {} : {}/{total}",
        format::bool(true),
        correct_count,
        format::bool(false),
        invalid_count,
        total = correct_count + invalid_count
    );
}

fn part2() {
    let mut correct_count = 0;
    let mut invalid_count = 0;

    for (_i, line) in file::lines().enumerate() {
        let was_correct = validate2(&line);
        // println!("{} [{}]: {}", i, format::bool(was_correct), line);
        if was_correct {
            correct_count += 1;
        } else {
            invalid_count += 1;
        }
    }

    println!("\nPart 2:");
    println!(
        "{}: {}/{total} | {} : {}/{total}",
        format::bool(true),
        correct_count,
        format::bool(false),
        invalid_count,
        total = correct_count + invalid_count
    );
}

fn validate(line: &String) -> bool {
    let (range, c, pwd) = parse_spec(line);
    let occurance = pwd.chars().filter(|s| *s == c).count();
    range.contains(&occurance)
}

fn validate2(line: &String) -> bool {
    let (range, c, pwd) = parse_spec(line);

    let char1 = pwd
        .chars()
        .nth(*range.start() - 1)
        .expect("expected positions to be valid");
    let char2 = pwd
        .chars()
        .nth(*range.end() - 1)
        .expect("expected positions to be valid");

    // print!("<{:?}> ", (char1, char2));

    char1 != char2 && (char1 == c || char2 == c)
}

fn parse_spec(spec: &String) -> (std::ops::RangeInclusive<usize>, char, &str) {
    let elements: Vec<&str> = (*spec).split_whitespace().collect();
    let pwd = elements[2];
    let range = parse_range(elements[0]);
    let c = elements[1]
        .chars()
        .next()
        .expect("Expected a character at position 2");
    (range, c, pwd)
}

fn parse_range(r: &str) -> std::ops::RangeInclusive<usize> {
    let mut elements = r.split("-").map(|n| {
        n.parse::<usize>()
            .expect("Needed to be a number in the range spec")
    });
    std::ops::RangeInclusive::new(elements.next().unwrap(), elements.next().unwrap())
}
