use std::str::FromStr;

use anyhow::{bail, Result};
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
struct History {
    values: Vec<i64>,
}

impl History {
    fn next_value_helper(values: &[i64]) -> Result<i64> {
        if values.iter().all(|x| *x == 0) {
            return Ok(0);
        }

        if values.len() < 2 {
            bail!("not enough values");
        }

        let mut next_values = Vec::default();
        for i in 1..values.len() {
            next_values.push(values[i] - values[i - 1]);
        }

        Ok(values[values.len() - 1] + Self::next_value_helper(&next_values)?)
    }

    fn prev_value_helper(values: &[i64]) -> Result<i64> {
        if values.iter().all(|x| *x == 0) {
            return Ok(0);
        }

        if values.len() < 2 {
            bail!("not enough values");
        }

        let mut prev_values = Vec::default();
        for i in 1..values.len() {
            prev_values.push(values[i] - values[i - 1]);
        }

        Ok(values[0] - Self::prev_value_helper(&prev_values)?)
    }

    fn next_value(&self) -> Result<i64> {
        Self::next_value_helper(&self.values)
    }

    fn prev_value(&self) -> Result<i64> {
        Self::prev_value_helper(&self.values)
    }
}

impl FromStr for History {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            values: s
                .split_whitespace()
                .map(|x| x.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct MirageMaintenance {
    histories: Vec<History>,
}

impl FromStr for MirageMaintenance {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            histories: s
                .lines()
                .map(History::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Problem for MirageMaintenance {
    const DAY: usize = 9;
    const TITLE: &'static str = "mirage maintenance";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self
            .histories
            .iter()
            .map(|x| x.next_value())
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self
            .histories
            .iter()
            .map(|x| x.prev_value())
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum())
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
        let solution = MirageMaintenance::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(2075724761, 1072));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = MirageMaintenance::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(114, 2));
    }
}
