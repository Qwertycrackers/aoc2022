use std::io::prelude::*;
use std::str::FromStr;
use num_bigint::BigUint;

pub fn monkey_business(mut input: impl BufRead) -> BigUint {
    let mut i = String::new();
    input.read_to_string(&mut i).unwrap();
    let mut monkeys = i.split("\n\n").map(Monkey::from_str).filter_map(Result::ok).collect::<Vec<_>>();
    let mut mailboxes: Vec<Vec<BigUint>> = vec![vec![]; monkeys.len()];

    for _ in 0..20 {
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            monkey.catch_items(mailboxes[i].drain(..));
            monkey.take_turn(&mut mailboxes);
        }
    }

    monkeys.sort_unstable_by(|m, n| m.inspections.cmp(&n.inspections));
    &monkeys.last().unwrap().inspections * &monkeys[monkeys.len() - 2].inspections
}

pub fn monkey_business_max_worry(mut input: impl BufRead) -> BigUint {
    let mut i = String::new();
    input.read_to_string(&mut i).unwrap();
    let mut monkeys = i.split("\n\n").map(Monkey::from_str).filter_map(Result::ok).collect::<Vec<_>>();
    let mut mailboxes: Vec<Vec<BigUint>> = vec![vec![]; monkeys.len()];

    for _ in 0..10000 {
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            monkey.catch_items(mailboxes[i].drain(..));
            monkey.take_turn_max_worry(&mut mailboxes);
        }
    }

    monkeys.sort_unstable_by(|m, n| m.inspections.cmp(&n.inspections));
    &monkeys.last().unwrap().inspections * &monkeys[monkeys.len() - 2].inspections
}

struct Monkey {
    inspections: BigUint,
    items: Vec<BigUint>,
    op: Operation,
    test: Test,
}
 impl Monkey {

     fn catch_items(&mut self, items: impl Iterator<Item = BigUint>) {
        self.items.extend(items)
     }

     fn take_turn(&mut self, mailboxes: &mut Vec<Vec<BigUint>>) {
        self.items.drain(..).for_each(|item| {
            self.inspections += 1u8;
            let worry = self.op.eval(item) / 3u8;
            let target = self.test.eval(&worry);
            mailboxes[target].push(worry);
        })
     }

     fn take_turn_max_worry(&mut self, mailboxes: &mut Vec<Vec<BigUint>>) {
        self.items.drain(..).for_each(|item| {
            self.inspections += 1u8;
            let worry = self.op.eval(item);
            let target = self.test.eval(&worry);
            mailboxes[target].push(worry);
        })
     }

 }

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_usize = |s: &str| s.split(' ').find_map(|s| usize::from_str(s).ok());
        let mut ls = s.lines();
        ls.next(); // Dump first line because they are in order
        match (
            ls.next().map(|s| s.split(' ').map(|s| s.trim_end_matches(',')).map(BigUint::from_str).filter_map(Result::ok).collect::<Vec<_>>()),
            ls.next().map(Operation::from_str),
            ls.next().and_then(first_usize),
            ls.next().and_then(first_usize),
            ls.next().and_then(first_usize),
        ) {
            (
                Some(items),
                Some(Ok(op)),
                Some(div_by),
                Some(if_true),
                Some(if_false),
            ) => Ok(Monkey {
                inspections: 0u8.into(),
                items,
                op,
                test: Test {
                    div_by,
                    if_true,
                    if_false
                }
            }),
            _ => Err(()),
        }
    }
}

struct Operation {
    op_type: OpType,
    l: OpArg,
    r: OpArg
}

impl Operation {
    fn eval(&self, item: BigUint) -> BigUint {
        let f = match self.op_type {
            OpType::Times => |a, b| a * b,
            OpType::Plus => |a, b| a + b
        };

        f(self.l.eval(&item), self.r.eval(&item))
    }
}
impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ls = s.split(' ').rev();
        let find_optype = |s: &str| match s.trim() {
            "+" => Ok(OpType::Plus),
            "*" => Ok(OpType::Times),
            _ => Err(())
        };

        let parse_arg = |s: &str| match (s.trim(), BigUint::from_str(s)) {
            (_, Ok(val)) => Some(OpArg::Val(val)),
            ("old", _) => Some(OpArg::Old),
            _ => None
        };

        match (ls.next().and_then(parse_arg), ls.next().map(find_optype), ls.next().and_then(parse_arg)) {
            (Some(r), Some(Ok(op_type)), Some(l)) => Ok(Self { op_type, l, r }),
            _ => Err(())
        }
    }
}

enum OpType {
    Plus,
    Times
}

enum OpArg {
    Val(BigUint),
    Old
}

impl OpArg {
    fn eval<'a>(&'a self, item: &'a BigUint) -> &'a BigUint {
       match self {
            Self::Val(v) => v,
            Self::Old => item,
       }
    }
}

struct Test {
    div_by: usize,
    if_true: usize,
    if_false: usize
}

impl Test {
    fn eval(&self, item: &BigUint) -> usize {
        if item % self.div_by == 0u8.into() {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let case = std::io::Cursor::new(
b"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1");
        assert_eq!(monkey_business(case), 10605u64.into())
    }

    #[test]
    fn example_2() {
        let case = std::io::Cursor::new(
b"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1");
        assert_eq!(monkey_business_max_worry(case), BigUint::from_str("2713310158").unwrap())
    }
}
