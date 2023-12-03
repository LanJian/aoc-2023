use crate::grid::Coordinate;
use std::str;
use std::str::FromStr;

use aoc_plumbing::Problem;
use rustc_hash::{FxHashMap, FxHashSet};

mod grid;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
struct PartNumber {
    start: Coordinate,
    number: usize,
}

#[derive(Debug, Clone)]
pub struct GearRatios {
    coords_to_part_numbers: FxHashMap<Coordinate, PartNumber>,
    symbol_coords: FxHashSet<Coordinate>,
    gear_coords: FxHashSet<Coordinate>,
}

impl GearRatios {
    fn part_numbers_sum(&self) -> usize {
        let mut part_numbers = FxHashSet::default();

        for coord in &self.symbol_coords {
            for neighbour in coord.neighbours() {
                if let Some(x) = self.coords_to_part_numbers.get(&neighbour) {
                    part_numbers.insert(*x);
                }
            }
        }

        part_numbers.iter().map(|x| x.number).sum()
    }

    fn gear_ratios_sum(&self) -> usize {
        let mut sum = 0;

        for coord in &self.gear_coords {
            let mut adjacent_parts = FxHashSet::default();

            for neighbour in coord.neighbours() {
                if let Some(x) = self.coords_to_part_numbers.get(&neighbour) {
                    adjacent_parts.insert(*x);
                }
            }

            if adjacent_parts.len() == 2 {
                sum += adjacent_parts
                    .into_iter()
                    .map(|x| x.number)
                    .product::<usize>();
            }
        }

        sum
    }
}

impl FromStr for GearRatios {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords_to_part_numbers = FxHashMap::default();
        let mut symbol_coords = FxHashSet::default();
        let mut gear_coords = FxHashSet::default();

        for (i, line) in s.lines().enumerate() {
            let mut left = 0;
            let mut prev_is_digit = false;

            let bytes = line.as_bytes();
            for (j, cur) in bytes.iter().enumerate() {
                if cur.is_ascii_digit() {
                    if !prev_is_digit {
                        left = j;
                    }

                    prev_is_digit = true;
                    continue;
                }

                if prev_is_digit {
                    let number: usize = str::from_utf8(&bytes[left..j])?.parse()?;
                    let part_number = PartNumber {
                        start: (i, left).into(),
                        number,
                    };

                    for k in left..j {
                        coords_to_part_numbers.insert((i, k).into(), part_number);
                    }
                }

                if !(*cur == b'.') {
                    symbol_coords.insert((i, j).into());
                    if *cur == b'*' {
                        gear_coords.insert((i, j).into());
                    }
                }

                left = j;
                prev_is_digit = false;
            }

            if prev_is_digit {
                let number: usize = str::from_utf8(&bytes[left..bytes.len()])?.parse()?;
                let part_number = PartNumber {
                    start: (i, left).into(),
                    number,
                };

                for k in left..bytes.len() {
                    coords_to_part_numbers.insert((i, k).into(), part_number);
                }
            }
        }

        Ok(Self {
            coords_to_part_numbers,
            symbol_coords,
            gear_coords,
        })
    }
}

impl Problem for GearRatios {
    const DAY: usize = 3;
    const TITLE: &'static str = "gear ratios";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.part_numbers_sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.gear_ratios_sum())
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
        let solution = GearRatios::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(531561, 83279367));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = GearRatios::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(4361, 467835));
    }

    #[test]
    fn example_two() {
        let input = "..#789";
        let solution = GearRatios::solve(input).unwrap();
        assert_eq!(solution, Solution::new(789, 0));
    }
}
