use std::io::prelude::*;
use std::collections::BTreeSet;
use std::str::FromStr;

pub fn total_positions_visited<'a>(input: impl BufRead + 'a, knots: usize) -> usize {
    let steps = parse_steps(input);
    let heads = Box::new(head_positions(steps));
    let tails = tails_n(heads, knots - 1);
    let visited = positions_visited(tails);
    visited.len()
}

fn tails_n<'a>(positions: Box<dyn Iterator<Item = Position> + 'a>, n: usize) -> Box<dyn Iterator<Item = Position> + 'a> {
    if n <= 0 { positions } else { tails_n(Box::new(tail_positions(positions)), n - 1) }
}

type Position = (i32, i32);

pub fn parse_steps<'a>(input: impl BufRead + 'a) -> impl Iterator<Item = Position> + 'a {
    input.lines().filter_map(Result::ok).flat_map(|line| {
        let mut words = line.split(' ');
        let (dir, n) = match (words.next(), words.next().map(i32::from_str).and_then(Result::ok)) {
            (Some("L"), Some(n)) => ((-1, 0), n),
            (Some("R"), Some(n)) => ((1, 0), n),
            (Some("U"), Some(n)) => ((0, 1), n),
            (Some("D"), Some(n)) => ((0, -1), n),
            _ => ((0, 0), 0),
        };
        (0..n).map(move |_| dir)
    })
}

pub fn head_positions<'a>(steps: impl Iterator<Item = Position> + 'a) -> impl Iterator<Item = Position> + 'a {
    let init = (0, 0);
    std::iter::once(init).chain(steps.scan(init, |pos, v| {
        *pos = (pos.0 + v.0, pos.1 + v.1);
        Some(*pos)
    }))
}

pub fn tail_positions<'a>(heads: impl Iterator<Item = Position> + 'a) -> impl Iterator<Item = Position> + 'a {
    let init = (0, 0);
    let delta = |(a, b), (x, y)| match (a - x, b - y) {
        (0, 0) => (0, 0),
        (0, d) if d > 1 => (0, 1),
        (0, d) if d < -1 => (0, -1),
        (c, 0) if c > 1 => (1, 0),
        (c, 0) if c < -1 => (-1, 0),
        (c, d) if c > 0 && d > 1 => (1, 1),
        (c, d) if c < 0 && d > 1 => (-1, 1),
        (c, d) if c > 0 && d < -1 => (1, -1),
        (c, d) if c < 0 && d < -1 => (-1, -1),
        (c, d) if c > 1 && d < 0 => (1, -1),
        (c, d) if c > 1 && d > 0 => (1, 1),
        (c, d) if c < -1 && d > 0 => (-1, 1),
        (c, d) if c < -1 && d < 0 => (-1, -1),
        _ => (0, 0),
    };
    std::iter::once(init).chain(heads.scan(init, move |pos, hpos| {
        let d = delta(hpos, *pos);
        *pos = (pos.0 + d.0, pos.1 + d.1);
        Some(*pos)
    }))
}

pub fn positions_visited(pos: impl Iterator<Item = Position>) -> BTreeSet<Position> {
    pos.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let case = std::io::Cursor::new(
b"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");
        assert_eq!(total_positions_visited(case, 2), 13);
    }

    #[test]
    fn example_2() {
        let case = std::io::Cursor::new(
b"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");
        assert_eq!(total_positions_visited(case, 10), 1);
    }

    #[test]
    fn example_3() {
        let case = std::io::Cursor::new(
b"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");
        assert_eq!(total_positions_visited(case, 10), 36);
    }
}
