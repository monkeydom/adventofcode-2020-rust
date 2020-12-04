use ansi_term::Colour::{Cyan, Green};
use ansi_term::Style;
use ansi_term::{ANSIString, ANSIStrings};

const NAME: &'static str = env!("CARGO_PKG_NAME");

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub fn preamble() {
    let name = some_kind_of_uppercase_first_letter(&NAME[..3]);
    let number = &NAME[3..];

    let line: &[ANSIString<'static>] = &[
        Style::new().bold().paint("2020 "),
        Cyan.bold().paint(name),
        Style::new().bold().paint(format!(" {}", number)),
    ];

    let ansi_line = ANSIStrings(&line);
    println!("{}", &ansi_line);
    let line_length = line.iter().fold(0, |acc, l| acc + l.len());

    println!("{}", Style::new().bold().paint("=".repeat(line_length)));
}
 
fn print_solution(s: &str, part: i8) {
    let line: &[ANSIString] = &[
        Style::new().bold().underline().paint(format!("Solution Part {}:", part)),
        Style::new().paint(" "),
        Green.bold().paint(s)
    ];

    let ansi_line = ANSIStrings(&line);
    println!("{}", ansi_line);
}

pub fn print_solution1(s: &str) {
    print_solution(s,1);
}

pub fn print_solution2(s: &str) {
    print_solution(s,2);
}
