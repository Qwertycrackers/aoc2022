use std::io::prelude::*;
use std::str::FromStr;

pub fn dirs_leq(input: impl BufRead, leq: usize) -> usize {
    input
        .lines()
        .filter_map(Result::ok)
        .filter_map(|x| match Cmd::from_str(&x) {
            Err(_) => {
                eprintln!("Not a command: {}", &x);
                None
            }
            x => x.ok(),
        })
        .chain(std::iter::repeat(Cmd::CdUp))
        .scan(vec![], |stack, cmd| match cmd {
            Cmd::CdDown => {
                stack.push(0);
                Some(None)
            }
            Cmd::CdUp => {
                let item = stack.pop();
                if let (Some(item), Some(parent)) = (item, stack.last_mut()) {
                    *parent += item;
                }
                item.map(|x| Some(x))
            }
            Cmd::File(size) => {
                if let Some(parent) = stack.last_mut() {
                    *parent += size;
                }
                Some(None)
            }
            _ => {
                Some(None)
            }
        })
        .filter_map(|x| x)
        .filter(|x| *x <= leq)
        .sum()
}

#[derive(Clone, Copy)]
enum Cmd {
    CdDown,
    CdUp,
    Ls,
    Dir,
    File(usize),
}

impl FromStr for Cmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.trim() {
            "$ cd .." => Ok(Cmd::CdUp),
            s if s.starts_with("$ cd") => Ok(Cmd::CdDown),
            s if s.starts_with("$ ls") => Ok(Cmd::Ls),
            s if s.starts_with("dir") => Ok(Cmd::Dir),
            s => s
                .split(' ')
                .next()
                .and_then(|x| usize::from_str(x).map(|x| Cmd::File(x)).ok())
                .ok_or(()),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let case = std::io::Cursor::new(
b"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        );
        assert_eq!(dirs_leq(case, 100000), 95437)
    }
}
