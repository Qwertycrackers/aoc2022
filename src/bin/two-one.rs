

use std::io;
use aoc22::two::eval_strategy;

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    println!("{}", eval_strategy(stdin));
}

