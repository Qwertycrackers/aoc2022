use std::io::prelude::*;
use std::str::FromStr;
use itertools::Itertools;

pub fn eval_strategy(input: impl BufRead) -> isize {
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

pub fn eval_full_strategy(input: impl BufRead) -> isize {
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

const WIN: isize = 6;
const LOSE: isize = 0;
const TIE: isize = 3;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Outcome {
    Win = WIN,
    Lose = LOSE,
    Tie = TIE,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Win),
            _ => Err(())
        }

    }
}

fn eval_pair(RpsPair(p): RpsPair) -> isize {
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
    }) + p.1 as isize
}

fn eval_strat_pair(StratPair((r, outcome)): StratPair) -> isize {
    r.compliment(outcome) as isize + outcome as isize
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl Rps {
    fn compliment(&self, outcome: Outcome) -> Rps {
        let matchups = [Rps::Rock, Rps::Paper, Rps::Scissors];
        let find = |(a, b): (&Rps, &Rps)| if *a == *self { Some(*b) } else { None };
        if outcome == Outcome::Tie {
            *self
        } else {
            if outcome == Outcome::Win { matchups.iter().cycle().tuple_windows().find_map(find) }
            else { matchups.iter().rev().cycle().tuple_windows().find_map(find) }
                    .unwrap_or(*self)
        }

    }
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

struct StratPair((Rps, Outcome));

impl FromStr for StratPair {
    type Err = ();

    fn from_str(s: &str) -> Result<StratPair, ()> {
        let mut letters = s.trim().split(' ');
        match (letters.next().map(Rps::from_str), letters.next().map(Outcome::from_str)) {
            (Some(Ok(a)), Some(Ok(b))) => Ok(StratPair((a, b))),
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

    #[test]
    fn example_2() {
        let case = std::io::Cursor::new(b"A Y\nB X\nC Z");
        assert_eq!(eval_full_strategy(case), 12)
    }
}
