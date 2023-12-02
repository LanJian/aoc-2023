use std::str::FromStr;

use anyhow::{bail, Result};
use aoc_plumbing::Problem;

#[derive(Debug, Clone, Default)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeSet {
    fn max_with(&mut self, other: &Self) {
        self.red = self.red.max(other.red);
        self.green = self.green.max(other.green);
        self.blue = self.blue.max(other.blue);
    }
}

impl FromStr for CubeSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut ret = CubeSet::default();

        for token in s.split(", ") {
            if let Some((left, right)) = token.split_once(' ') {
                match right {
                    "red" => ret.red = left.parse()?,
                    "green" => ret.green = left.parse()?,
                    "blue" => ret.blue = left.parse()?,
                    _ => bail!("could not cube color"),
                }
            } else {
                bail!("could not parse cube set")
            }
        }

        Ok(ret)
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: usize,
    max_cube_set: CubeSet,
}

impl Game {
    fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        self.max_cube_set.red <= red && self.max_cube_set.green <= green && self.max_cube_set.blue <= blue
    }

    fn minimum_set_power(&self) -> usize {
        self.max_cube_set.red * self.max_cube_set.green * self.max_cube_set.blue
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((left, right)) = s.split_once(": ") {
            let id = left[5..].parse()?;
            let mut max_cube_set = CubeSet::default();

            for token in right.split("; ") {
                max_cube_set.max_with(&CubeSet::from_str(token)?);
            }

            Ok(Game { id, max_cube_set })
        } else {
            bail!("could not parse game")
        }
    }
}

#[derive(Debug, Clone)]
pub struct CubeConundrum {
    games: Vec<Game>,
}

impl CubeConundrum {
    fn possible_ids_sum(&self, red: usize, green: usize, blue: usize) -> usize {
        self.games
            .iter()
            .filter_map(|x| x.is_possible(red, green, blue).then_some(x.id))
            .sum()
    }

    fn minimum_set_power_sum(&self) -> usize {
        self.games.iter().map(|x| x.minimum_set_power()).sum()
    }
}

impl FromStr for CubeConundrum {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let games = s
            .lines()
            .map(|x| Game::from_str(x))
            .collect::<Result<Vec<Game>>>()?;
        Ok(Self { games })
    }
}

impl Problem for CubeConundrum {
    const DAY: usize = 2;
    const TITLE: &'static str = "cube conundrum";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.possible_ids_sum(12, 13, 14))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.minimum_set_power_sum())
    }
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = CubeConundrum::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(2256, 74229));
    }

    #[test]
    fn example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let solution = CubeConundrum::solve(input).unwrap();
        assert_eq!(solution, Solution::new(8, 2286));
    }
}
