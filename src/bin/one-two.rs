
use std::io;
use aoc22::one::greatest_n_calories;

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    println!("{}", greatest_n_calories(stdin, 3));
}

