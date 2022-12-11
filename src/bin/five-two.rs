use std::io;
use aoc22::five::eval_crates_2;

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    println!("{}", eval_crates_2(stdin));
}

