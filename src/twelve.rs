use nalgebra as na;
use std::io::prelude::*;
use std::collections::VecDeque;

pub fn shortest_path(input: impl BufRead) -> Option<u32> {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let h = input
        .lines()
        .filter_map(Result::ok)
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (x, y);
                        0
                    }
                    'E' => {
                        end = (x, y);
                        25
                    }
                    c => c as u8 - 'a' as u8,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = h.len();
    let width = h.first().map(|f| f.len()).unwrap_or(0);

    let hmap: na::DMatrix<u8> = na::DMatrix::from_row_iterator(
        height,
        width,
        h.into_iter().flat_map(|g| g.into_iter()),
    );
    let mut visited: na::DMatrix<bool> = na::DMatrix::from_element(height, width, false);
    let mut heads = VecDeque::with_capacity(20);
    heads.push_back((0, start));

    let get = |h, (x, y)| Some((0usize, 0usize));
    loop {
        if let Some((h, (x, y))) = heads.pop_front() {
            get(h, (x + 1, y)).into_iter()
            .chain(get(h, (x - 1, y)).into_iter())
            .chain(get(h, (x, y + 1)).into_iter())
            .chain(get(h, (x, y - 1)).into_iter())
            .filter_map(|a| a)
            .find_map(|coords| {
                if coords == end {
                    Some(coords);
                } else {
                    heads.push_back((h + 1, coords));
                    None
                }
            })
        } else {
            break None;
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let case = std::io::Cursor::new(
            b"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        );

        assert_eq!(shortest_path(case), 31)
    }
}
