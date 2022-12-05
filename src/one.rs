use std::io::prelude::*;

pub fn greatest_calories(input: impl BufRead) -> i32 {
    input.lines().scan(0, |sum, el| {
        let el = el.unwrap();
        eprintln!("el: {}, sum: {}", el, sum);
        match i32::from_str_radix(el.trim(), 10) {
            Ok(eln) => {
                eprintln!("Adding: {}", eln);
                *sum += eln;
                Some(None)
            }
            Err(_) if el.trim().is_empty() => {
                let total = *sum;
                *sum = 0;
                eprintln!("Yielding: {}", total);
                Some(Some(total))
            }
            _ => {
                eprintln!("Parsing err! Neither empty string nor integer: {}", el);
                Some(None)
            }
        }
    }).filter_map(|x| x).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::{io, fs};
    use super::*;
    
    fn case_path(case: &str) -> String {
        let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        format!("{}/{}/{}", cargo_dir, "cases", case)
    }

    #[test]
    fn prob_example() {
        let case = io::BufReader::new(fs::File::open(case_path("1-1-example")).unwrap());
        assert_eq!(greatest_calories(case), 24000)
    }

    #[test]
    fn simple_example() {
        let case = io::Cursor::new(b"\n2000\n500\n500\n\n2500\n\n");
        assert_eq!(greatest_calories(case), 3000)
    }
}
