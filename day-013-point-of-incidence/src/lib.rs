use std::fmt;
use std::str::FromStr;

use anyhow::bail;
use aoc_plumbing::Problem;
use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Pattern {
    rows: Vec<u32>,
    cols: Vec<u32>,
    original_inflection: usize,
}

impl Pattern {
    fn inflection_with_smudge(&mut self) -> usize {
        for i in 0..self.rows.len() {
            for j in 0..self.cols.len() {
                self.rows[i] ^= 1 << (self.cols.len() - j - 1);
                self.cols[j] ^= 1 << (self.rows.len() - i - 1);

                let result = self.inflection();
                if result != self.original_inflection && result > 0 {
                    return result;
                }

                self.rows[i] ^= 1 << (self.cols.len() - j - 1);
                self.cols[j] ^= 1 << (self.rows.len() - i - 1);
            }
        }

        0
    }

    fn inflection(&self) -> usize {
        let mut ret = 0;
        let mut j = 0;
        let mut reflecting = false;

        for (i, &x) in self.rows.iter().enumerate().skip(1) {
            if reflecting {
                if x == self.rows[j] {
                    if j == 0 {
                        if 100 * ret == self.original_inflection {
                            reflecting = false;
                            j = 0;
                            ret = 0;
                        } else {
                            return 100 * ret;
                        }
                    }
                    j -= 1;
                } else {
                    reflecting = false;
                    ret = 0;
                }
            }

            if x == self.rows[i - 1] {
                ret = i;
                j = i - 1;
                reflecting = true;

                if j == 0 {
                    if 100 == self.original_inflection {
                        reflecting = false;
                        j = 0;
                        ret = 0;
                    } else {
                        return 100;
                    }
                }

                j -= 1;
            }
        }

        if ret > 0 && 100 * ret != self.original_inflection {
            return 100 * ret;
        }

        reflecting = false;
        j = 0;
        ret = 0;

        for (i, &x) in self.cols.iter().enumerate().skip(1) {
            if reflecting {
                if x == self.cols[j] {
                    if j == 0 {
                        if ret == self.original_inflection {
                            reflecting = false;
                            j = 0;
                            ret = 0;
                        } else {
                            return ret;
                        }
                    }
                    j -= 1;
                } else {
                    reflecting = false;
                    ret = 0;
                }
            }

            if x == self.cols[i - 1] {
                ret = i;
                j = i - 1;
                reflecting = true;

                if j == 0 {
                    if 1 == self.original_inflection {
                        reflecting = false;
                        j = 0;
                        ret = 0;
                    } else {
                        return 1;
                    }
                }

                j -= 1;
            }
        }

        ret
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
            original_inflection: 0,
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
            sum += result;
        }
        Ok(sum)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self
            .patterns
            .par_iter_mut()
            .map(|x| x.inflection_with_smudge())
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
}
