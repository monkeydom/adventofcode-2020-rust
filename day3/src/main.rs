mod file;
mod format;

fn main() {
    println!("2020 Day 2");
    println!("==========");

    part1();

    part2();
}

fn part1() {
    println!("\nPart 1:");


    for line in file::lines() {
        println!("{}", line);
    }
}

fn part2() {
    println!("\nPart 2:");

}