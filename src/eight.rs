use std::io::prelude::*;
use std::cmp;

pub fn visible_trees(input: impl BufRead) -> usize {
    let trees = parse_trees(input);
    let y = trees.len();
    let x = trees.first().map(|x| x.len()).unwrap_or(0);
    let mut visible: Visible = vec![vec![false; x]; y];
    visible.first_mut().map(|x| *x = vec![true; x.len()]);
    visible.last_mut().map(|x| *x = vec![true; x.len()]);

    for i in 1..(y-1) {
        visible[i].first_mut().map(|x| *x = true); // Outside is visible by definition
        let mut max = trees[i].first().copied().unwrap_or(0);
        for j in 1..(x-1) {
           let tree = trees[i][j];
           visible[i][j] |= tree > max;
           max = cmp::max(tree, max);
        }
    }

    for i in 1..(y-1) {
        visible[i].last_mut().map(|x| *x = true); // Outside is visible by definition
        let mut max = trees[i].last().copied().unwrap_or(0);
        for j in (1..(x-1)).rev() {
           let tree = trees[i][j];
           visible[i][j] |= tree > max;
           max = cmp::max(tree, max);
        }
    }

    for j in 1..(x-1) {
        let mut max = trees.first().map(|f| f[j]).unwrap_or(0);
        for i in 1..(y-1) {
           let tree = trees[i][j];
           visible[i][j] |= tree > max;
           max = cmp::max(tree, max);
        }
    }

    for j in 1..(x-1) {
        let mut max = trees.last().map(|f| f[j]).unwrap_or(0);
        for i in (1..(y-1)).rev() {
           let tree = trees[i][j];
           visible[i][j] |= tree > max;
           max = cmp::max(tree, max);
        }
    }

    count_visible(&visible)
}

pub fn max_scenic_score(input: impl BufRead) -> usize {
    let trees = parse_trees(input);
    let y = trees.len();
    let x = trees.first().map(|x| x.len()).unwrap_or(0);
    let mut max_score = 0;

    for i in 0..y {
        for j in 0..x {
            let tree = trees[i][j];
            let mut len_up = 0;
            for k in (0..i).rev() {
                len_up += 1;
                if trees[k][j] >= tree { break; }
            }
            let mut len_down = 0;
            for k in (i+1)..y {
                len_down += 1;
                if trees[k][j] >= tree { break; }
            }
            let mut len_left = 0;
            for l in (0..j).rev() {
                len_left += 1;
                if trees[i][l] >= tree { break; }
            }
            let mut len_right = 0;
            for l in (j+1)..x {
                len_right += 1;
                if trees[i][l] >= tree { break; }
            }
            let score = len_up * len_down * len_left * len_right;
            max_score = cmp::max(max_score, score)
        }
    }
    
    max_score
}

type Trees = Vec<Vec<u32>>;
type Visible = Vec<Vec<bool>>;

fn parse_trees(input: impl BufRead) -> Trees {
    input.lines().filter_map(Result::ok).map(parse_treeline).collect()
}

fn parse_treeline(s: String) -> Vec<u32> {
    s.chars().filter_map(|x| x.to_digit(10)).collect()
}

fn count_visible(visible: &Visible) -> usize {
    visible.into_iter().flat_map(|v| v.into_iter()).filter(|x| **x).count()
}

#[cfg(test)]
pub mod eight {
    use super::*;

    #[test]
    fn example_1() {
        let case = std::io::Cursor::new(
b"30373
25512
65332
33549
35390");
        assert_eq!(visible_trees(case), 21);
    }

    #[test]
    fn example_2() {
        let case = std::io::Cursor::new(
b"30373
25512
65332
33549
35390");
        assert_eq!(max_scenic_score(case), 8);
    }

}

