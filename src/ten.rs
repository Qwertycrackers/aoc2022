use std::io::prelude::*;
use std::str::FromStr;
use itertools::Itertools;

pub fn sum_signals(input: impl BufRead, start: usize, interval: usize) -> i32 {
    let instrs = parse_instrs(input);
    let xregs = xregs(instrs);
    let signals = signals(xregs, start, interval);
    signals.sum()
}

pub fn crt_image<'a>(input: impl BufRead + 'a) -> String {
    let instrs = parse_instrs(input);
    let xregs = xregs(instrs).chunks(40);
    itertools::join(xregs.into_iter().map(|l| {
        l.into_iter().enumerate().map(|(pos, x)| if x.abs_diff(pos as i32) <= 1 { '#' } else { '.' }).collect::<String>()
    }), "\n")
}

enum Instr {
    Addx(i32),
    Noop
}

fn parse_instrs<'a>(input: impl BufRead + 'a) -> impl Iterator<Item = Instr> + 'a {
    input.lines().filter_map(Result::ok).map(|line| {
        let mut words = line.split(' ');
        match (words.next(), words.next().map(i32::from_str)) {
            (Some("addx"), Some(Ok(x))) => Ok(Instr::Addx(x)),
            (Some("noop"), _) => Ok(Instr::Noop),
            _ => Err(()),
        }
    })
    .filter_map(Result::ok)
}

fn xregs<'a>(is: impl Iterator<Item = Instr> + 'a) -> impl Iterator<Item = i32> + 'a {
    is.flat_map(|i| {
        let (latency, delta) = match i {
            Instr::Addx(x) => (2, x),
            Instr::Noop => (1, 0),
        };
        (0..(latency-1)).map(|_| 0).chain(std::iter::once(delta))
    })
    .scan(1, |x, delta| {
        let now = *x;
        *x += delta;
        Some(now)
    })
}

fn signals(xregs: impl Iterator<Item = i32>, start: usize, interval: usize) -> impl Iterator<Item = i32> {
    xregs.enumerate().skip(start - 1).step_by(interval).map(|(count, x)| (count + 1) as i32 * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn case() -> std::io::Cursor<&'static [u8]> {
        std::io::Cursor::new(
b"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop")
    }

    #[test]
    fn example_1() {
        assert_eq!(sum_signals(case(), 20, 40), 13140)
    }

    #[test]
    fn example_2() {
        let res = 
"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(crt_image(case()), res)
    }
}
