use std::fmt;
use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, bail, Result};
use aoc_common::{
    direction::CardinalDirection,
    grid::{Coordinate, Grid},
};
use aoc_plumbing::Problem;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Tile {
    fn connects(&self, dir: &CardinalDirection) -> bool {
        matches!(
            (self, dir),
            (Self::NS, CardinalDirection::North)
                | (Self::NS, CardinalDirection::South)
                | (Self::EW, CardinalDirection::East)
                | (Self::EW, CardinalDirection::West)
                | (Self::NE, CardinalDirection::North)
                | (Self::NE, CardinalDirection::East)
                | (Self::NW, CardinalDirection::North)
                | (Self::NW, CardinalDirection::West)
                | (Self::SW, CardinalDirection::South)
                | (Self::SW, CardinalDirection::West)
                | (Self::SE, CardinalDirection::South)
                | (Self::SE, CardinalDirection::East)
                | (Self::Start, _)
        )
    }
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let ret = match value {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => bail!("could not parse tile"),
        };

        Ok(ret)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileKind {
    Unknown,
    Loop(Tile),
    Inside,
    Outside,
}

impl Default for TileKind {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for TileKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            TileKind::Loop(x) => match x {
                Tile::NS => '│',
                Tile::EW => '─',
                Tile::NW => '┘',
                Tile::NE => '└',
                Tile::SW => '┐',
                Tile::SE => '┌',
                Tile::Start => 'S',
                _ => '.',
            },
            TileKind::Inside => 'x',
            _ => '.',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone)]
pub struct PipeMaze {
    grid: Grid<Tile>,
    start: Coordinate,
}

impl PipeMaze {
    fn connects(&self, coord: &Coordinate, dir: &CardinalDirection) -> bool {
        if !self.grid.is_in_bounds(coord.neighbour(dir)) {
            return false;
        }

        let tile = self.grid[*coord];
        let other = self.grid[coord.neighbour(dir)];

        match dir {
            CardinalDirection::North => {
                tile.connects(&CardinalDirection::North)
                    && other.connects(&CardinalDirection::South)
            }
            CardinalDirection::South => {
                tile.connects(&CardinalDirection::South)
                    && other.connects(&CardinalDirection::North)
            }
            CardinalDirection::West => {
                tile.connects(&CardinalDirection::West) && other.connects(&CardinalDirection::East)
            }
            CardinalDirection::East => {
                tile.connects(&CardinalDirection::East) && other.connects(&CardinalDirection::West)
            }
        }
    }

    fn determine_start_tile(&self) -> Result<Tile> {
        if self
            .grid
            .get(self.start.north())
            .is_some_and(|x| x.connects(&CardinalDirection::South))
        {
            if self
                .grid
                .get(self.start.south())
                .is_some_and(|x| x.connects(&CardinalDirection::North))
            {
                Ok(Tile::NS)
            } else if self
                .grid
                .get(self.start.west())
                .is_some_and(|x| x.connects(&CardinalDirection::East))
            {
                Ok(Tile::NW)
            } else if self
                .grid
                .get(self.start.east())
                .is_some_and(|x| x.connects(&CardinalDirection::West))
            {
                Ok(Tile::NE)
            } else {
                bail!("invalid start tile")
            }
        } else if self
            .grid
            .get(self.start.south())
            .is_some_and(|x| x.connects(&CardinalDirection::North))
        {
            if self
                .grid
                .get(self.start.west())
                .is_some_and(|x| x.connects(&CardinalDirection::East))
            {
                Ok(Tile::SW)
            } else if self
                .grid
                .get(self.start.east())
                .is_some_and(|x| x.connects(&CardinalDirection::West))
            {
                Ok(Tile::SE)
            } else {
                bail!("invalid start tile")
            }
        } else {
            Ok(Tile::EW)
        }
    }

    fn inside(&self) -> Result<usize> {
        let mut memo = Grid::new(self.grid.n, self.grid.m, TileKind::Unknown);

        // populate the loop
        let mut q = VecDeque::default();
        let mut visited = FxHashSet::default();
        q.push_back(self.start);

        while !q.is_empty() {
            let coord = q.pop_front().unwrap();

            if !self.grid.is_in_bounds(coord) {
                continue;
            }

            if visited.contains(&coord) {
                continue;
            }

            visited.insert(coord);
            memo[coord] = if self.grid[coord] == Tile::Start {
                TileKind::Loop(self.determine_start_tile()?)
            } else {
                TileKind::Loop(self.grid[coord])
            };

            for dir in CardinalDirection::all() {
                if self.connects(&coord, &dir) {
                    q.push_back(coord.neighbour(&dir));
                }
            }
        }

        // test and fill tiles
        let mut count = 0;
        for i in 0..self.grid.n {
            for j in 0..self.grid.m {
                let coord = (i, j).into();

                if memo[coord] != TileKind::Unknown {
                    continue;
                }

                let kind = self.check(&coord, &memo);
                let filled_count = self.fill(&coord, &mut memo, &kind);

                if kind == TileKind::Inside {
                    count += filled_count;
                }
            }
        }

        Ok(count)
    }

    fn fill(&self, source: &Coordinate, memo: &mut Grid<TileKind>, kind: &TileKind) -> usize {
        let mut count = 0;
        let mut q = VecDeque::default();
        q.push_back(*source);

        while !q.is_empty() {
            let coord = q.pop_front().unwrap();

            if !memo.is_in_bounds(coord) {
                continue;
            }

            if memo[coord] != TileKind::Unknown {
                continue;
            }

            memo[coord] = *kind;
            count += 1;

            q.extend(coord.cardinal_neighbours());
        }

        count
    }

    fn check(&self, source: &Coordinate, memo: &Grid<TileKind>) -> TileKind {
        let mut count = 0;
        let mut prev = None;
        for i in source.row() as usize..memo.n {
            let coord = (i, source.col() as usize).into();
            let tile = memo[coord];

            match tile {
                TileKind::Loop(Tile::EW) => {
                    count += 1;
                    prev = None;
                }
                TileKind::Loop(Tile::SW) => {
                    count += 1;
                    prev = Some(Tile::SW);
                }
                TileKind::Loop(Tile::SE) => {
                    count += 1;
                    prev = Some(Tile::SE);
                }
                TileKind::Loop(Tile::NW) => {
                    if prev == Some(Tile::SW) {
                        count += 1;
                    }
                    prev = None;
                }
                TileKind::Loop(Tile::NE) => {
                    if prev == Some(Tile::SE) {
                        count += 1;
                    }
                    prev = None;
                }
                _ => (),
            }
        }

        // even means outside, odd means inside
        if count % 2 == 0 {
            TileKind::Outside
        } else {
            TileKind::Inside
        }
    }

    fn max_distance(&self) -> Result<usize> {
        let mut q = VecDeque::default();
        let mut visited = FxHashSet::default();
        let mut max_dist = 0;
        q.push_back((self.start, 0));

        while !q.is_empty() {
            let (coord, dist) = q.pop_front().unwrap();

            if !self.grid.is_in_bounds(coord) {
                continue;
            }

            if visited.contains(&coord) {
                continue;
            }

            visited.insert(coord);

            if dist > max_dist {
                max_dist = dist;
            }

            for dir in CardinalDirection::all() {
                if self.connects(&coord, &dir) {
                    q.push_back((coord.neighbour(&dir), dist + 1));
                }
            }
        }

        Ok(max_dist)
    }
}

impl FromStr for PipeMaze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        let start = grid
            .find_coordinate(|x| *x == Tile::Start)
            .ok_or_else(|| anyhow!("could not find start location"))?;
        Ok(Self { grid, start })
    }
}

impl Problem for PipeMaze {
    const DAY: usize = 10;
    const TITLE: &'static str = "pipe maze";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.max_distance()
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.inside()
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
        let solution = PipeMaze::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(7066, 401));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = PipeMaze::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(8, 1));
    }

    #[test]
    fn example_two() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let mut instance = PipeMaze::instance(input).unwrap();
        assert_eq!(instance.part_two().unwrap(), 10);
    }
}
