use std::{io, fs};
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ElfCalories {
    provisions: Vec<i64>
}

impl ElfCalories {
    pub fn total() -> i64 {
        todo!()
    }
}

pub fn greatest_calories(input: impl BufRead) -> i64 {
    todo!()
}

pub fn parse_elves(input: impl BufRead) -> impl Iterator<Item = ElfCalories> {
    input.lines().filter_map(|r| r.ok()).scan(Vec::with_capacity(10), |st, el| match (st, el) {
        (st, "") if st.empty() => None
        (st, el) => {
            if let Some(el) = i64.from_str(&el).ok() {
                st.push_back(el)
            }
            None
        }

    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn case_path(case: &str) -> String {
        let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        format!("{}/{}/{}", cargo_dir, "cases", case)
    }

    #[test]
    fn prob_example() {
        let case = io::BufReader::new(fs::File::open(case_path("1-1-example")).unwrap());
        assert_eq!(greatest_calories(case), 24000)
    }

    #[test]
    fn parses() {
        let case = io::BufReader::new(fs::File::open(case_path("1-1-example")).unwrap());
        let elves = parse_elves(case).collect::<Vec<_>>();
        let should_elves = vec![
            ElfCalories { provisions: vec![1000, 2000, 3000] },
            ElfCalories { provisions: vec![4000] },
            ElfCalories { provisions: vec![5000, 6000] },
            ElfCalories { provisions: vec![7000, 8000, 9000] },
            ElfCalories { provisions: vec![10000] }
        ];
        assert_eq!(elves, should_elves)
    }
}
