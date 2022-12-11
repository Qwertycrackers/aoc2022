use std::io;
use aoc22::four::count_containing;

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    println!("{}", count_containing(stdin));
}



