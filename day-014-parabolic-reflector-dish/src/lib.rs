use std::str::FromStr;

use anyhow::bail;
use aoc_common::{
    direction::CardinalDirection,
    grid::{Coordinate, Grid},
};
use aoc_plumbing::Problem;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Tile {
    Round,
    Cube,
    Empty,
}

use std::fmt;

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Round => 'O',
            Self::Cube => '#',
            Self::Empty => '.',
        };

        write!(f, "{}", c)
    }
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'O' => Self::Round,
            '#' => Self::Cube,
            '.' => Self::Empty,
            _ => bail!("invalid tile"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ParabolicReflectorDish {
    platform: Grid<Tile>,
}

impl ParabolicReflectorDish {
    fn total_load(&self) -> usize {
        let mut total = 0;

        for j in 0..self.platform.m {
            for i in 0..self.platform.n {
                let coord = (i, j).into();
                if self.platform[coord] == Tile::Round {
                    total += self.platform.n - i;
                }
            }
        }

        total
    }
    fn cycle(&mut self, cycles: usize) {
        let mut cache = FxHashMap::default();
        let mut period = 0;
        let mut start = 0;

        // first find the cycle start and period
        for i in 0..cycles {
            self.tilt(CardinalDirection::North);
            self.tilt(CardinalDirection::West);
            self.tilt(CardinalDirection::South);
            self.tilt(CardinalDirection::East);

            if i == cycles - 1 {
                return;
            }

            if let Some(&x) = cache.get(&self.platform.grid) {
                period = i - x;
                start = x;
                break;
            } else {
                cache.insert(self.platform.grid.clone(), i);
            }
        }

        // then jump ahead and process the remaining cycles
        let remaining = (cycles - start - 1) % period;
        for _ in 0..remaining {
            self.tilt(CardinalDirection::North);
            self.tilt(CardinalDirection::West);
            self.tilt(CardinalDirection::South);
            self.tilt(CardinalDirection::East);
        }
    }

    fn tilt(&mut self, dir: CardinalDirection) {
        match dir {
            CardinalDirection::North => self.tilt_helper(true, false),
            CardinalDirection::South => self.tilt_helper(true, true),
            CardinalDirection::West => self.tilt_helper(false, false),
            CardinalDirection::East => self.tilt_helper(false, true),
        }
    }

    fn tilt_helper(&mut self, col_major: bool, rev: bool) {
        let (outer, inner) = match col_major {
            true => (self.platform.m, self.platform.n),
            false => (self.platform.n, self.platform.m),
        };

        for i in 0..outer {
            let mut target = if rev { inner - 1 } else { 0 };

            for jj in 0..inner {
                let j = if rev { inner - jj - 1 } else { jj };

                let coord: Coordinate = match col_major {
                    true => (j, i).into(),
                    false => (i, j).into(),
                };

                match self.platform[coord] {
                    Tile::Round => {
                        let target_coord = match col_major {
                            true => (target, i).into(),
                            false => (i, target).into(),
                        };

                        self.platform[target_coord] = Tile::Round;

                        if coord != target_coord {
                            self.platform[coord] = Tile::Empty;
                        }

                        match rev {
                            true => target = target.saturating_sub(1),
                            false => target += 1,
                        }
                    }
                    Tile::Cube => match rev {
                        true => target = j.saturating_sub(1),
                        false => target = j + 1,
                    },
                    Tile::Empty => (),
                }
            }
        }
    }
}

impl FromStr for ParabolicReflectorDish {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            platform: Grid::from_str(s)?,
        })
    }
}

impl Problem for ParabolicReflectorDish {
    const DAY: usize = 14;
    const TITLE: &'static str = "parabolic reflector dish";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.tilt(CardinalDirection::North);
        Ok(self.total_load())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.cycle(1_000_000_000);
        Ok(self.total_load())
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
        let solution = ParabolicReflectorDish::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(108935, 100876));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = ParabolicReflectorDish::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(136, 64));
    }
}
