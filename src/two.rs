use std::io::prelude::*;
use std::str::FromStr;

pub fn eval_strategy(input: impl BufRead) -> u32 {
    input.lines().filter_map(Result::ok).map(|s| {
        match RpsPair::from_str(&s) {
            Err(_) => {
                eprintln!("Failed parse: {}", s);
                0
            }
            Ok(p) => eval_pair(p)
        }
    })
    .sum()
}

const WIN: u32 = 6;
const LOSE: u32 = 0;
const TIE: u32 = 3;

fn eval_pair(RpsPair(p): RpsPair) -> u32 {
    use Rps::*;
    match p {
        (a, b) if a == b => TIE,
        (Rock, Paper) => WIN,
        (Rock, Scissors) => LOSE,
        (Paper, Rock) => LOSE,
        (Paper, Scissors) => WIN,
        (Scissors, Rock) => LOSE,
        (Scissors, Paper) => WIN,
        _ => TIE, // Impossible, compiler is stupid
    }
}

#[derive(PartialEq, Eq)]
pub enum Rps {
    Rock,
    Paper,
    Scissors
}

impl FromStr for Rps {
    type Err = ();

    fn from_str(s: &str) -> Result<Rps, ()> {
        match s {
            "A" | "X" => Ok(Rps::Rock),
            "B" | "Y" => Ok(Rps::Paper),
            "C" | "Z" => Ok(Rps::Scissors),
            _ => Err(())
        }
    }
}

struct RpsPair((Rps, Rps));

impl FromStr for RpsPair { 
    type Err = ();

    fn from_str(s: &str) -> Result<RpsPair, ()> {
        Ok(Self((Rps::Rock, Rps::Rock)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let case = std::io::Cursor::new(b"A Y\nB X\nC Z");
        assert_eq!(eval_strategy(case), 15)
    }
}
