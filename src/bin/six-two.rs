use std::io;
use aoc22::six::first_n_count;

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    println!("{}", first_n_count(stdin, 14));
}

