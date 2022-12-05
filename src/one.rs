use std::collections::btree_set::BTreeSet;
use std::io::prelude::*;

pub fn greatest_calories(input: impl BufRead) -> i32 {
    parse_calories(input).max().unwrap_or(0)
}

pub fn greatest_n_calories(input: impl BufRead, n: usize) -> i32 {
    let cals = parse_calories(input).collect::<BTreeSet<i32>>();
    cals.into_iter().rev().take(n).sum()
}

pub fn parse_calories(input: impl BufRead) -> impl Iterator<Item = i32> {
    input.lines().scan(0, |sum, el| {
        let el = el.unwrap();
        match i32::from_str_radix(el.trim(), 10) {
            Ok(eln) => {
                *sum += eln;
                Some(None)
            }
            Err(_) if el.trim().is_empty() => {
                let total = *sum;
                *sum = 0;
                Some(Some(total))
            }
            _ => {
                Some(None)
            }
        }
    }).filter_map(|x| x)
}

#[cfg(test)]
mod tests {
    use std::{io, fs};
    use super::*;
    use crate::util::*;
    
    #[test]
    fn prob_example() {
        let case = io::BufReader::new(fs::File::open(case_path("1-1-example")).unwrap());
        assert_eq!(greatest_calories(case), 24000)
    }

    #[test]
    fn simple_example() {
        let case = io::Cursor::new(b"\n2000\n500\n500\n\n2500\n\n");
        assert_eq!(greatest_calories(case), 3000)
    }

    #[test]
    fn n_example() {
        let case = io::BufReader::new(fs::File::open(case_path("1-1-example")).unwrap());
        assert_eq!(greatest_n_calories(case, 3), 45000)
    }
}
