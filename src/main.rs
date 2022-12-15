use std::io;
use aoc22::{seven, eight, nine, ten, eleven, eleven_two, twelve};

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    match std::env::args().skip(1).next().as_ref().map(String::as_str) {
        Some("seven-one") => println!("{}", seven::dirs_leq(stdin, 100000)),
        Some("seven-two") => println!("{}", seven::least_dir_geq(stdin, 30000000, 70000000)),
        Some("eight-one") => println!("{}", eight::visible_trees(stdin)),
        Some("eight-two") => println!("{}", eight::max_scenic_score(stdin)),
        Some("nine-one") => println!("{}", nine::total_positions_visited(stdin, 2)),
        Some("nine-two") => println!("{}", nine::total_positions_visited(stdin, 10)),
        Some("ten-one") => println!("{}", ten::sum_signals(stdin, 20, 40)),
        Some("ten-two") => println!("{}", ten::crt_image(stdin)),
        Some("eleven-one") => println!("{}", eleven::monkey_business(stdin)),
        Some("eleven-two") => println!("{}", eleven_two::monkey_business(stdin)),
        Some("twelve-one") => println!("{}", twelve::shortest_path(stdin)),
        _ => ()
    }
}




