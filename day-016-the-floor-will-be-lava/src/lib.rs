use std::str::FromStr;

use anyhow::bail;
use aoc_common::{
    direction::CardinalDirection,
    grid::{Coordinate, Grid},
};
use aoc_plumbing::Problem;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileKind {
    Empty,
    VSplit,
    HSplit,
    FMirror,
    BMirror,
}

impl TryFrom<char> for TileKind {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '|' => Self::VSplit,
            '-' => Self::HSplit,
            '/' => Self::FMirror,
            '\\' => Self::BMirror,
            _ => bail!("invalid tile"),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    kind: TileKind,
    energized: bool,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: value.try_into()?,
            energized: false,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TheFloorWillBeLava {
    grid: Grid<Tile>,
}

impl TheFloorWillBeLava {
    fn total_energized(&self) -> usize {
        let mut total = 0;

        for i in 0..self.grid.n {
            for j in 0..self.grid.m {
                if self.grid[(i, j).into()].energized {
                    total += 1;
                }
            }
        }

        total
    }

    fn max_energized(&mut self) -> usize {
        let mut total = 0;
        let mut visited = FxHashSet::default();

        for i in 0..self.grid.n {
            self.energize_helper(&(i, 0).into(), &CardinalDirection::East, &mut visited);
            total = total.max(self.total_energized());
            visited.clear();
            self.clear();

            self.energize_helper(
                &(i, self.grid.m - 1).into(),
                &CardinalDirection::West,
                &mut visited,
            );
            total = total.max(self.total_energized());
            visited.clear();
            self.clear();
        }

        for j in 0..self.grid.m {
            self.energize_helper(&(0, j).into(), &CardinalDirection::South, &mut visited);
            total = total.max(self.total_energized());
            visited.clear();
            self.clear();

            self.energize_helper(
                &(self.grid.n - 1, j).into(),
                &CardinalDirection::North,
                &mut visited,
            );
            total = total.max(self.total_energized());
            visited.clear();
            self.clear();
        }

        total
    }

    fn clear(&mut self) {
        for i in 0..self.grid.n {
            for j in 0..self.grid.m {
                let tile = &mut self.grid[(i, j).into()];
                tile.energized = false;
            }
        }
    }

    fn energize(&mut self) {
        self.energize_helper(
            &(0_isize, 0_isize).into(),
            &CardinalDirection::East,
            &mut FxHashSet::default(),
        );
    }

    fn energize_helper(
        &mut self,
        position: &Coordinate,
        dir: &CardinalDirection,
        visited: &mut FxHashSet<(Coordinate, CardinalDirection)>,
    ) {
        if !self.grid.is_in_bounds(*position) {
            return;
        }

        if visited.contains(&(*position, *dir)) {
            return;
        }

        let tile = &mut self.grid[*position];
        tile.energized = true;
        visited.insert((*position, *dir));

        if tile.kind == TileKind::Empty {
            return self.energize_helper(&position.neighbour(dir), dir, visited);
        }

        if tile.kind == TileKind::VSplit {
            if *dir == CardinalDirection::North || *dir == CardinalDirection::South {
                return self.energize_helper(&position.neighbour(dir), dir, visited);
            }

            self.energize_helper(&position.north(), &CardinalDirection::North, visited);
            self.energize_helper(&position.south(), &CardinalDirection::South, visited);
            return;
        }

        if tile.kind == TileKind::HSplit {
            if *dir == CardinalDirection::East || *dir == CardinalDirection::West {
                return self.energize_helper(&position.neighbour(dir), dir, visited);
            }

            self.energize_helper(&position.east(), &CardinalDirection::East, visited);
            self.energize_helper(&position.west(), &CardinalDirection::West, visited);
            return;
        }

        if tile.kind == TileKind::FMirror {
            match dir {
                CardinalDirection::North => {
                    self.energize_helper(&position.east(), &CardinalDirection::East, visited)
                }
                CardinalDirection::South => {
                    self.energize_helper(&position.west(), &CardinalDirection::West, visited)
                }
                CardinalDirection::East => {
                    self.energize_helper(&position.north(), &CardinalDirection::North, visited)
                }
                CardinalDirection::West => {
                    self.energize_helper(&position.south(), &CardinalDirection::South, visited)
                }
            }

            return;
        }

        match dir {
            CardinalDirection::North => {
                self.energize_helper(&position.west(), &CardinalDirection::West, visited)
            }
            CardinalDirection::South => {
                self.energize_helper(&position.east(), &CardinalDirection::East, visited)
            }
            CardinalDirection::East => {
                self.energize_helper(&position.south(), &CardinalDirection::South, visited)
            }
            CardinalDirection::West => {
                self.energize_helper(&position.north(), &CardinalDirection::North, visited)
            }
        }
    }
}

impl FromStr for TheFloorWillBeLava {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: Grid::from_str(s)?,
        })
    }
}

impl Problem for TheFloorWillBeLava {
    const DAY: usize = 16;
    const TITLE: &'static str = "the floor will be lava";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.energize();
        Ok(self.total_energized())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.max_energized())
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
        let solution = TheFloorWillBeLava::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(8901, 9064));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = TheFloorWillBeLava::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(46, 51));
    }
}
