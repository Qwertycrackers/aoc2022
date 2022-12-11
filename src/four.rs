
use std::io::prelude::*;

pub fn count_containing(input: impl BufRead) -> usize {
    input.lines()
        .filter_map(Result::ok)
        .filter_map(|s| match parse_range_pair(&s) {
            Err(_) => {
                eprintln!("Failed parse! {}", s);
                None
            }
            Ok((l, r)) if l.contains(&r) || r.contains(&l) => Some(()),
            Ok(_) => None
        })
        .count()
}

pub fn count_overlapping(input: impl BufRead) -> usize {
    input.lines()
        .filter_map(Result::ok)
        .filter_map(|s| match parse_range_pair(&s) {
            Err(_) => {
                eprintln!("Failed parse! {}", s);
                None
            }
            Ok((l, r)) if l.overlaps(&r) => Some(()),
            Ok(_) => None
        })
        .count()
}

fn parse_range_pair(s: &str) -> Result<(Assignment, Assignment), ()> {
    let mut s = s.split(',');
    match (s.next().map(parse_range), s.next().map(parse_range), s.next()) {
        (Some(Ok(l)), Some(Ok(r)), None) => Ok((l, r)),
        _ => Err(()),
    }

}

fn parse_range(s: &str) -> Result<Assignment, ()> {
    let mut s = s.split('-');
    let parse_num = |x| i32::from_str_radix(x, 10);
    match (s.next().map(parse_num), s.next().map(parse_num), s.next()) {
        (Some(Ok(start)), Some(Ok(end)), None) => Ok(Assignment { start, end }),
        _ => Err(()),
    }
}

struct Assignment {
    start: i32,
    end: i32,
}

impl Assignment {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        !(self.start > other.end || self.end < other.start)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let case = std::io::Cursor::new(
b"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        assert_eq!(count_containing(case), 2)
    }

    #[test]
    fn example_2() {
        let case = std::io::Cursor::new(
b"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
        assert_eq!(count_overlapping(case), 4)
    }
}
