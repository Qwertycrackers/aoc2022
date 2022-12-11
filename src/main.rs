use std::io;
use aoc22::seven::dirs_leq;

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    match std::env::args().skip(1).next().as_ref().map(String::as_str) {
        Some("seven-one") => println!("{}", dirs_leq(stdin, 100000)),
        _ => ()
    }
}




