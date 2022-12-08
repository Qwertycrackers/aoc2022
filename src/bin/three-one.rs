use std::io;
use aoc22::three::sum_shared_priorities;

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    println!("{}", sum_shared_priorities(stdin));
}

