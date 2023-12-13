use std::fmt;
use std::str::FromStr;

use anyhow::bail;
use aoc_plumbing::Problem;
use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Pattern {
    rows: Vec<u32>,
    cols: Vec<u32>,
    original_inflection: Option<usize>,
}

impl Pattern {
    fn inflection_with_smudge(&mut self) -> Option<usize> {
        for i in 0..self.rows.len() {
            for j in 0..self.cols.len() {
                self.rows[i] ^= 1 << (self.cols.len() - j - 1);
                self.cols[j] ^= 1 << (self.rows.len() - i - 1);

                let result = self.inflection();
                if result.is_some() && result != self.original_inflection {
                    return result;
                }

                self.rows[i] ^= 1 << (self.cols.len() - j - 1);
                self.cols[j] ^= 1 << (self.rows.len() - i - 1);
            }
        }

        None
    }

    fn inflection(&self) -> Option<usize> {
        self.inflection_helper(&self.rows, 100)
            .or_else(|| self.inflection_helper(&self.cols, 1))
    }

    fn inflection_helper(&self, slice: &[u32], factor: usize) -> Option<usize> {
        let n = slice.len();

        for i in 1..=slice.len() / 2 {
            if (0..i).all(|j| slice[j] == slice[2 * i - j - 1]) {
                let ret = Some(factor * i);
                if ret != self.original_inflection {
                    return ret;
                }
            }

            if (0..i).all(|j| slice[n - i + j] == slice[n - i - 1 - j]) {
                let ret = Some(factor * (n - i));
                if ret != self.original_inflection {
                    return ret;
                }
            }
        }

        None
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            writeln!(f, "{:0width$b}", row, width = self.cols.len())?;
        }

        Ok(())
    }
}

impl FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::default();
        let mut cols = Vec::default();

        for (i, line) in s.lines().enumerate() {
            let mut row = 0;

            for (j, c) in line.chars().enumerate() {
                row = row << 1
                    | match c {
                        '.' => 0,
                        '#' => 1,
                        _ => bail!("invalid char"),
                    };

                if i == 0 {
                    cols.push(0);
                }

                cols[j] = cols[j] << 1
                    | match c {
                        '.' => 0,
                        '#' => 1,
                        _ => bail!("invalid char"),
                    };
            }

            rows.push(row);
        }

        Ok(Self {
            rows,
            cols,
            original_inflection: None,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PointOfIncidence {
    patterns: Vec<Pattern>,
}

impl FromStr for PointOfIncidence {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let patterns = s
            .split("\n\n")
            .map(Pattern::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { patterns })
    }
}

impl Problem for PointOfIncidence {
    const DAY: usize = 13;
    const TITLE: &'static str = "point of incidence";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let mut sum = 0;
        for pattern in self.patterns.iter_mut() {
            let result = pattern.inflection();
            pattern.original_inflection = result;

            if let Some(x) = result {
                sum += x;
            }
        }

        Ok(sum)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self
            .patterns
            .par_iter_mut()
            .map(|x| x.inflection_with_smudge().unwrap_or_default())
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
        let solution = PointOfIncidence::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(35691, 39037));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = PointOfIncidence::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(405, 400));
    }

    #[test]
    fn example_two() {
        let input = "##..#.######..##.
.###..##...#.#..#
#.#..#..##.#..#.#
####.#..###..##..
......##.########
......##.###.####
####.#..###..##..
#.#..#..##.#..#.#
.###..##...#.#..#
##..#.######..##.
##..#.######..##.

.#.##....
.##.##...
.###.#...
.##...###
......###
..#...###
.##...###
.###.#...
.##.##...
.#.##....
.########
##.#..###
##...####";

        let solution = PointOfIncidence::solve(input).unwrap();
        assert_eq!(solution, Solution::new(1008, 1000));
    }

    #[test]
    fn example_three() {
        let input = "....#.#.#.##..#
##...#..#..#.##
####.##..##..##
#.###.#.##.#.#.
..#.##.#...####
..#.##.#...####
#.###.#.##...#.
#.#..#..###...#
#.#..#..###...#
#.###.#.##...#.
..#.##.#...####
..#.##.#...####
#.###.#.##.#.#.

.....#..##..#
##...########
.#.#....##...
#.#.#........
.#...########
...##########
######..##..#";

        let solution = PointOfIncidence::solve(input).unwrap();
        assert_eq!(solution, Solution::new(809, 1111));
    }
}
