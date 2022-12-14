use std::io::prelude::*;
use std::str::FromStr;
use reikna::factor;

pub fn monkey_business(mut input: impl BufRead) -> usize {
    let mut i = String::new();
    input.read_to_string(&mut i).unwrap();
    let mut monkeys = i.split("\n\n").map(Monkey::from_str).filter_map(Result::ok).collect::<Vec<_>>();
    let mut mailboxes: Vec<Vec<Worry>> = vec![vec![]; monkeys.len()];
    let lcm = monkeys.iter().map(|m| m.test.div_by).reduce(factor::lcm).unwrap_or(1);
    eprintln!("Monkeys: {:#?}, lcm: {}", monkeys, lcm);

    for _n in 0..10000 {
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            monkey.catch_items(mailboxes[i].drain(..));
            monkey.take_turn(&mut mailboxes, &lcm);
        }
        if (_n + 1) % 1000 == 0 || _n == 0 || _n == 19 {
            eprintln!("== After round {} ==", _n + 1);
            for (i, monkey) in monkeys.iter().enumerate() {
                eprintln!("Monkey {} inspected items {} times.", i, monkey.inspections);
                //eprintln!("{:#?}", monkey);
                //eprintln!("{:?}", mailboxes[i]);
            }
        }
    }

    monkeys.sort_unstable_by_key(|m| m.inspections);
    monkeys.last().unwrap().inspections * monkeys[monkeys.len() - 2].inspections
}

type Worry = u64;

#[derive(Debug)]
struct Monkey {
    inspections: usize,
    items: Vec<Worry>,
    op: Operation,
    test: Test,
}
 impl Monkey {

     fn catch_items(&mut self, items: impl Iterator<Item = Worry>) {
        self.items.extend(items)
     }

     fn take_turn(&mut self, mailboxes: &mut Vec<Vec<Worry>>, lcm: &Worry) {
        self.items.drain(..).for_each(|item| {
            self.inspections += 1;
            let worry = self.op.eval(item) % *lcm;
            let target = self.test.eval(&worry);
            mailboxes[target].push(worry);
        })
     }

 }

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_u64 = |s: &str| s.split(' ').find_map(|s| u64::from_str(s).ok());
        let mut ls = s.lines();
        ls.next(); // Dump first line because they are in order
        match (
            ls.next().map(|s| s.split(' ').map(|s| s.trim_end_matches(',')).map(u64::from_str).filter_map(Result::ok).collect::<Vec<_>>()),
            ls.next().map(Operation::from_str),
            ls.next().and_then(first_u64),
            ls.next().and_then(first_u64),
            ls.next().and_then(first_u64),
        ) {
            (
                Some(items),
                Some(Ok(op)),
                Some(div_by),
                Some(if_true),
                Some(if_false),
            ) => Ok(Monkey {
                inspections: 0u8.into(),
                items: items.into_iter().collect(),
                op,
                test: Test {
                    div_by,
                    if_true: if_true as usize,
                    if_false: if_false as usize
                }
            }),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Operation {
    op_type: OpType,
    l: OpArg,
    r: OpArg
}

impl Operation {
    fn eval(&self, item: Worry) -> Worry {
        use OpType::*;
        use OpArg::*;
        match (&self.op_type, &self.l, &self.r) {
            (Times, Val(v), Old) | (Times, Old, Val(v)) => {
                item * v
            }
            (Plus, Val(v), Old) | (Plus, Old, Val(v)) => {
                item + v
            }
            (Plus, Old, Old) => item + item,
            (Times, Old, Old) => item * item,
            (Plus, Val(a), Val(b)) => a + b,
            (Times, Val(a), Val(b)) => a * b,
        }
    }
}
impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut ls = s.split(' ').rev();
        let find_optype = |s: &str| match s.trim() {
            "+" => Ok(OpType::Plus),
            "*" => Ok(OpType::Times),
            _ => Err(())
        };

        let parse_arg = |s: &str| match (s.trim(), u64::from_str(s)) {
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

#[derive(Debug)]
enum OpType {
    Plus,
    Times
}

#[derive(Debug)]
enum OpArg {
    Val(Worry),
    Old
}

#[derive(Debug)]
struct Test {
    div_by: Worry,
    if_true: usize,
    if_false: usize
}

impl Test {
    fn eval(&self, worry: &Worry) -> usize {
        if *worry % self.div_by == 0 {
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
        assert_eq!(monkey_business(case), 2713310158)
    }
}

