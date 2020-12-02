mod file;
mod format;

fn main() {
    println!("2020 Day 2");
    println!("==========");

    let mut correct_count = 0;
    let mut invalid_count = 0;

    for (i, line) in file::lines().enumerate() {
        let was_correct = validate(&line);
        println!("{} [{}]: {}", i, format::bool(was_correct), line);
        if was_correct {
            correct_count += 1;
        } else {
            invalid_count += 1;
        }
    }

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
    let elements: Vec<&str> = (*line).split_whitespace().collect();
    let pwd = elements[2];
    let range = parse_range(elements[0]);
    let c = elements[1]
        .chars()
        .next()
        .expect("Expected a character at position 2");

    let occurance = pwd.chars().filter(|s| *s == c).count();

    // print!("<{:?}> ", (&occurance, &range, &c, &pwd));

    range.contains(&occurance)
}

fn parse_range(r: &str) -> std::ops::RangeInclusive<usize> {
    let mut elements = r.split("-").map(|n| {
        n.parse::<usize>()
            .expect("Needed to be a number in the range spec")
    });
    std::ops::RangeInclusive::new(elements.next().unwrap(), elements.next().unwrap())
}
