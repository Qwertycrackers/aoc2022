
use std::io::prelude::*;
use std::collections::VecDeque;

pub fn eval_crates(input: impl BufRead) -> String {
    let (mut crates, ops) = parse_crate_ops(input);
    // print_crates(&crates);
    ops.filter_map(|r| match r {
        Err(s) => {
            eprintln!("Failed op parse!: {}", s);
            None
        }
        r => r.ok()
    }).for_each(|op| apply_op(&mut crates, op));
    top_crates(&crates)
}

pub fn eval_crates_2(input: impl BufRead) -> String {
    let (mut crates, ops) = parse_crate_ops(input);
    // print_crates(&crates);
    ops.filter_map(|r| match r {
        Err(s) => {
            eprintln!("Failed op parse!: {}", s);
            None
        }
        r => r.ok()
    }).for_each(|op| apply_op_2(&mut crates, op));
    top_crates(&crates)
}

fn parse_crate_ops(input: impl BufRead) -> (Crates, impl Iterator<Item = Result<Op, String>>) {
    let mut lines = input.lines().filter_map(Result::ok);
    let mut crates = Vec::new();
    
    for line in lines.by_ref() {
        if line.trim().is_empty() { break; }
        let ncols = (line.len() + 1) / 4;
        crates.resize_with(ncols, || VecDeque::with_capacity(10));
        for col in 0..ncols {
            let item = line.as_bytes()[col * 4 + 1] as char;
            if item.is_ascii_digit() { break; }
            if item.is_alphabetic() {
                crates[col].push_back(item);
            }
        }

    }

    (crates, lines.map(parse_op))
}

fn parse_op(s: String) -> Result<Op, String> {
    let mut nums = s.split(' ').filter_map(|s| usize::from_str_radix(s, 10).ok());

    match (nums.next(), nums.next(), nums.next(), nums.next()) {
        (Some(a), Some(b), Some(c), None) => Ok(Op(a, b - 1, c - 1)),
        _ => Err(s),
    }
}

type Crates = Vec<VecDeque<char>>;

fn apply_op(crates: &mut Crates, Op(n, from, to): Op) {
    let item = crates[from].pop_front();
    if let Some(item) = item {
        crates[to].push_front(item);
    }
    if n > 1 {
        apply_op(crates, Op(n - 1, from, to))
    }
}

fn apply_op_2(crates: &mut Crates, Op(n, from, to): Op) {
    let mut items: VecDeque<char> = crates[from].drain(0..n).collect();
    items.extend(crates[to].iter());
    crates[to] = items;
}

fn top_crates(crates: &Crates) -> String {
    crates.iter().filter_map(|x| x.front()).collect()
}

fn _print_crates(crates: &Crates) {
    crates.iter().for_each(|c| eprintln!("{}", c.iter().collect::<String>()));
}

#[derive(Clone, Copy)]
struct Op(usize, usize, usize);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let case = std::io::Cursor::new(
b"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2");

        assert_eq!(eval_crates(case).as_str(), "CMZ")
    }

    #[test]
    fn example_2() {
        let case = std::io::Cursor::new(
b"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2");

        assert_eq!(eval_crates_2(case).as_str(), "MCD")
    }
}

