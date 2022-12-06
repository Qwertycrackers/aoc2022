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

pub fn eval_full_strategy(input: impl BufRead) -> u32 {
    input.lines().filter_map(Result::ok).map(|s| {
        match StratPair::from_str(&s) {
            Err(_) => {
                eprintln!("Failed parse: {}", s);
                0
            }
            Ok(p) => eval_strat_pair(p)
        }
    })
    .sum()
}

const WIN: u32 = 6;
const LOSE: u32 = 0;
const TIE: u32 = 3;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Outcome {
    Win = WIN,
    LOSE = LOSE,
    Tie = TIE,
}

fn eval_pair(RpsPair(p): RpsPair) -> u32 {
    use Rps::*;
    (match p {
        (a, b) if a == b => TIE,
        (Rock, Paper) => WIN,
        (Rock, Scissors) => LOSE,
        (Paper, Rock) => LOSE,
        (Paper, Scissors) => WIN,
        (Scissors, Rock) => WIN,
        (Scissors, Paper) => LOSE,
        _ => TIE, // Impossible, compiler is stupid
    }) + p.1 as u32
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3
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
        let mut letters = s.trim().split(' ');
        match (letters.next().map(Rps::from_str), letters.next().map(Rps::from_str)) {
            (Some(Ok(a)), Some(Ok(b))) => Ok(RpsPair((a, b))),
            _ => Err(())
        }
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
