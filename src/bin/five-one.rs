use std::io;
use aoc22::five::eval_crates;

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    println!("{}", eval_crates(stdin));
}
