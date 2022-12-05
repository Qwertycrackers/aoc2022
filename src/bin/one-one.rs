
use std::io;
use aoc22::one::greatest_calories;

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    println!("{}", greatest_calories(stdin));
}
