use std::str::FromStr;

use anyhow::{anyhow, Ok, Result};
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
struct Calibration {
    text: String,
}

impl Calibration {
    const WORDS: [&'static str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn recover(&self) -> Result<u32> {
        let mut iter = self.text.chars();

        let first_digit = iter
            .find_map(|c| c.to_digit(10))
            .ok_or_else(|| anyhow!("could not find first digit"))?;
        let last_digit = iter
            .rev()
            .find_map(|c| c.to_digit(10))
            .unwrap_or(first_digit);

        Ok(first_digit * 10 + last_digit)
    }

    fn recover_enhanced(&self) -> u32 {
        let mut first = 0;
        'outer: for i in 0..self.text.len() {
            for j in 0..Self::WORDS.len() {
                let word = Self::WORDS[j];
                if self.text.as_bytes()[i] == (j + 49) as u8 || self.text[i..].starts_with(word) {
                    first = j + 1;
                    break 'outer;
                }
            }
        }

        let mut last = 0;
        'outer: for i in (0..self.text.len()).rev() {
            for j in 0..Self::WORDS.len() {
                let word = Self::WORDS[j];
                if self.text.as_bytes()[i] == (j + 49) as u8 || self.text[i..].starts_with(word) {
                    last = j + 1;
                    break 'outer;
                }
            }
        }

        (first * 10 + last) as u32
    }
}

#[derive(Debug, Clone)]
pub struct Trebuchet {
    calibrations: Vec<Calibration>,
}

impl Trebuchet {
    fn recover(&self) -> Result<u32> {
        let mut ret = 0;

        for calibration in &self.calibrations {
            ret += calibration.recover()?;
        }

        Ok(ret)
    }

    fn recover_enhanced(&self) -> Result<u32> {
        let mut ret = 0;

        for calibration in &self.calibrations {
            ret += calibration.recover_enhanced();
        }

        Ok(ret)
    }
}

impl FromStr for Trebuchet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calibrations = s
            .lines()
            .map(|line| Calibration {
                text: line.to_owned(),
            })
            .collect();
        Ok(Self { calibrations })
    }
}

impl Problem for Trebuchet {
    const DAY: usize = 1;
    const TITLE: &'static str = "trebuchet";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u32;
    type P2 = u32;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.recover()
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.recover_enhanced()
    }
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = Trebuchet::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(54390, 54277));
    }

    #[test]
    fn example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let solution = Trebuchet::solve(input).unwrap();
        assert_eq!(solution, Solution::new(142, 142));
    }

    #[test]
    fn example_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let mut instance = Trebuchet::instance(input).unwrap();
        assert_eq!(instance.part_two().unwrap(), 281);
    }
}
