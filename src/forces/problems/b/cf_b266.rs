// Title: Queue at the School
// URL: https://codeforces.com/problemset/problem/266/B

use std::io::prelude::*;
use std::io::{self, BufReader};

pub struct Solution<'a> {
    input: BufReader<&'a mut dyn Read>,
    output: &'a mut dyn Write,
}

impl<'a> Solution<'a> {
    pub fn new(input: &'a mut dyn Read, output: &'a mut dyn Write) -> Self {
        Self {
            input: BufReader::new(input),
            output,
        }
    }

    pub fn get_input(&mut self) -> Option<String> {
        let mut input = String::new();
        if let Ok(eof) = self.input.read_line(&mut input) {
            if eof == 0 {
                return None;
            }

            Some(input.trim().to_string())
        } else {
            None
        }
    }

    pub fn print<T: std::fmt::Display>(&mut self, text: T) {
        writeln!(self.output, "{}", text).expect("Valid output");
    }
}

impl<'a> Solution<'a> {
    pub fn solve(&mut self) {
        let (n, t): (usize, usize) = match self
            .get_input()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>()[..]
        {
            [a, b] => (a, b),
            _ => panic!("Invalid input"),
        };
        let mut chars: Vec<char> = self.get_input().unwrap().chars().collect();

        for _ in 0..t {
            let mut i = 0;
            while i < n - 1 {
                if chars[i] == 'B' && chars[i + 1] == 'G' {
                    chars.swap(i, i + 1);
                    i += 2;
                } else {
                    i += 1;
                }
            }
        }

        self.print(chars.iter().collect::<String>())
    }
}

#[allow(dead_code)]
fn main() {
    let mut input = io::stdin();
    let mut output = io::stdout();
    let mut solution = Solution::new(&mut input, &mut output);
    solution.solve();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_solve(input: &str, output: &str) {
        let mut cursor = io::Cursor::new(input);
        let mut buf: Vec<u8> = Vec::new();

        let mut solution = Solution::new(&mut cursor, &mut buf);
        solution.solve();

        let res = String::from_utf8(buf).expect("valid string");

        assert_eq!(res, format!("{output}\n"));
    }

    #[test]
    fn test_1() {
        let input = "5 1
BGGBG";
        let output = "GBGGB";

        run_solve(input, output);
    }

    #[test]
    fn test_2() {
        let input = "5 2
BGGBG";
        let output = "GGBGB";

        run_solve(input, output);
    }

    #[test]
    fn test_3() {
        let input = "4 1
GGGB";
        let output = "GGGB";

        run_solve(input, output);
    }

    #[test]
    fn test_4() {
        let input = "8 3
BBGBGBGB";
        let output = "GGBGBBBB";

        run_solve(input, output);
    }
}
