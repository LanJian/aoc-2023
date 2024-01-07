use std::str::FromStr;

use aoc_common::grid::Coordinate;
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct CosmicExpansion {
    galaxies: Vec<Coordinate>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl CosmicExpansion {
    fn distance_between(&self, a: &Coordinate, b: &Coordinate, expansion: usize) -> usize {
        let mut dist = a.manhattan_distance(b);

        let top = self.empty_rows.partition_point(|&x| x <= a.row() as usize);
        let bottom = self.empty_rows.partition_point(|&x| x < b.row() as usize);
        let left = self.empty_cols.partition_point(|&x| x <= a.col() as usize);
        let right = self.empty_cols.partition_point(|&x| x < b.col() as usize);

        dist += bottom.abs_diff(top) * (expansion - 1);
        dist += right.abs_diff(left) * (expansion - 1);

        dist
    }

    fn total_distances(&self, expansion: usize) -> usize {
        let mut total = 0;
        for (i, a) in self.galaxies.iter().enumerate() {
            for b in self.galaxies[i + 1..].iter() {
                total += self.distance_between(a, b, expansion);
            }
        }

        total
    }
}

impl FromStr for CosmicExpansion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies = Vec::default();
        let mut empty_rows = Vec::default();
        let mut is_empty_cols = Vec::default();

        for (i, line) in s.lines().enumerate() {
            if i == 0 {
                is_empty_cols = vec![true; line.len()];
            }
            let mut is_empty_row = true;

            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    is_empty_row = false;
                    is_empty_cols[j] = false;
                    galaxies.push((i, j).into());
                }
            }

            if is_empty_row {
                empty_rows.push(i);
            }
        }

        let empty_cols = is_empty_cols
            .iter()
            .enumerate()
            .filter(|&(_, x)| *x)
            .map(|(j, _)| j)
            .collect();

        Ok(Self {
            galaxies,
            empty_rows,
            empty_cols,
        })
    }
}

impl Problem for CosmicExpansion {
    const DAY: usize = 11;
    const TITLE: &'static str = "cosmic expansion";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.total_distances(2))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.total_distances(1000000))
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
        let solution = CosmicExpansion::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(9556896, 685038186836));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = CosmicExpansion::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(374, 82000210));
    }
}
