mod file;

fn main() {
    println!("2020 Day 2");
    println!("==========");

    for (i, line) in file::lines().enumerate() {
        println!("{}: {}", i, line);
    }
}
