use std::str::FromStr;

use anyhow::{anyhow, bail, Ok, Result};
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
struct Calibration {
    text: String,
}

impl Calibration {
    const WORDS: [&'static str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    const REVERSED_WORDS: [&'static str; 9] = [
        "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];

    fn word_to_digit(word: &str) -> Result<u32> {
        match word {
            "one" | "eno" => Ok(1),
            "two" | "owt" => Ok(2),
            "three" | "eerht" => Ok(3),
            "four" | "ruof" => Ok(4),
            "five" | "evif" => Ok(5),
            "six" | "xis" => Ok(6),
            "seven" | "neves" => Ok(7),
            "eight" | "thgie" => Ok(8),
            "nine" | "enin" => Ok(9),
            _ => bail!("invalid word"),
        }
    }

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

    fn recover_enhanced(&self) -> Result<u32> {
        let first_digit_pair = self.text.chars().enumerate().find(|(_, c)| c.is_digit(10));
        let last_digit_pair = self
            .text
            .chars()
            .rev()
            .enumerate()
            .find(|(_, c)| c.is_digit(10));

        let mut first_word = None;
        let mut first_word_index = usize::MAX;
        for word in Self::WORDS {
            if let Some(index) = self.text.find(word) {
                if index < first_word_index {
                    first_word_index = index;
                    first_word = Some(word);
                }
            }
        }

        let mut last_word = None;
        let mut last_word_index = usize::MAX;
        let reversed_text: String = self.text.chars().rev().collect();
        for word in Self::REVERSED_WORDS {
            if let Some(index) = reversed_text.find(word) {
                if index < last_word_index {
                    last_word_index = index;
                    last_word = Some(word);
                }
            }
        }

        let first = match first_digit_pair {
            Some((index, c)) if index < first_word_index => c
                .to_digit(10)
                .ok_or_else(|| anyhow!("could not convert first digit"))?,
            _ => Self::word_to_digit(
                first_word.ok_or_else(|| anyhow!("could not find first digit"))?,
            )?,
        };

        let last = match last_digit_pair {
            Some((index, c)) if index < last_word_index => c
                .to_digit(10)
                .ok_or_else(|| anyhow!("could not convert last digit"))?,
            _ => {
                Self::word_to_digit(last_word.ok_or_else(|| anyhow!("could not find last digit"))?)?
            }
        };

        Ok(first * 10 + last)
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
            ret += calibration.recover_enhanced()?;
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
