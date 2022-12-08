use std::io;
use aoc22::two::eval_full_strategy;

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    println!("{}", eval_full_strategy(stdin));
}

