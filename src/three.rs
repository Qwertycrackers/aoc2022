use itertools::Itertools;
use std::collections::BTreeSet;
use std::io::prelude::*;
use std::str::FromStr;

pub fn sum_shared_priorities(input: impl BufRead) -> u32 {
    input
        .lines()
        .filter_map(Result::ok)
        .map(|s| match Rucksack::from_str(&s) {
            Err(_) => {
                eprintln!("Failed parse: {}", s);
                0
            }
            Ok(p) => {
                if let Some(a) = p.shared_priority() {
                    a
                } else {
                    eprintln!("No shared priority!: {}", s);
                    0
                }
            }
        })
        .sum()
}

pub fn sum_badges(input: impl BufRead) -> u32 {
    input
        .lines()
        .filter_map(Result::ok)
        .filter_map(|s| match Rucksack::from_str(&s) {
            Err(_) => {
                eprintln!("Failed parse: {}", s);
                None
            }
            Ok(p) => Some(p),
        })
        .tuples()
        .filter_map(|(a, b, c)| {
            a.total()
                .intersection(&b.total())
                .copied()
                .collect::<BTreeSet<char>>()
                .intersection(&c.total())
                .next()
                .copied()
                .map(priority)
        })
        .sum()
}

struct Rucksack {
    left: BTreeSet<char>,
    right: BTreeSet<char>,
}

impl Rucksack {
    pub fn shared_priority(&self) -> Option<u32> {
        self.left
            .intersection(&self.right)
            .next()
            .copied()
            .map(priority)
    }

    pub fn total(&self) -> BTreeSet<char> {
        self.left.union(&self.right).copied().collect()
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let midpoint = s.len() / 2;
        let l = &s[0..midpoint];
        let r = &s[midpoint..];
        Ok(Self {
            left: l.chars().collect(),
            right: r.chars().collect(),
        })
    }
}

fn priority(c: char) -> u32 {
    let upper = if c.is_ascii_uppercase() { 26 } else { 0 };
    let alphabet = (c.to_ascii_lowercase() as u32) - ('a' as u32) + 1;
    upper + alphabet
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let case = std::io::Cursor::new(
            b"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(157, sum_shared_priorities(case))
    }

    #[test]
    fn example_2() {
        let case = std::io::Cursor::new(
            b"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(70, sum_badges(case))
    }

    #[test]
    fn p_priority() {
        assert_eq!(priority('p'), 16)
    }

    #[test]
    fn p_upper_priority() {
        assert_eq!(priority('P'), 42)
    }

    #[test]
    fn l_upper_priority() {
        assert_eq!(priority('L'), 38)
    }
}
