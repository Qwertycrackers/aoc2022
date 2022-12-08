use std::io;
use aoc22::three::sum_badges;

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    println!("{}", sum_badges(stdin));
}


