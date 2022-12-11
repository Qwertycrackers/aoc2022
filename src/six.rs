use std::io::prelude::*;
use std::collections::BTreeSet;

pub fn first_n_count(input: impl Read, n: usize) -> usize {
    let input = input.bytes().filter_map(Result::ok).map(|x| x as char).collect::<Vec<_>>();
    let set = BTreeSet::<char>::new();
    input.windows(n).scan(set, |set, x| {
        // eprintln!("{} {}", set.iter().collect::<String>(), x.iter().collect::<String>());
        set.extend(x);
        if set.len() >= n { 
            Some(false) 
        } else { 
            set.clear();
            Some(true) 
        }
    })
    .take_while(|x| *x)
    .count() + n
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let case = std::io::Cursor::new(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(first_n_count(case, 4), 7)
    }

    #[test]
    fn example_2() {
        let case = std::io::Cursor::new(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(first_n_count(case, 14), 19)
    }

}
