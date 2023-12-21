use std::{str::FromStr, collections::BinaryHeap};

use anyhow::anyhow;
use aoc_common::grid::{Grid, Coordinate};
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
struct Block {
    value: u8,
}

impl TryFrom<char> for Block {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Self {
            value: value
                .to_digit(10)
                .map(|x| x as u8)
                .ok_or_else(|| anyhow!("invalid block value"))?,
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Node {
    dist: usize,
    coord: Coordinate,
}

impl From<(usize, Coordinate)> for Node {
    fn from(value: (usize, Coordinate)) -> Self {
        Self {
            dist: value.0,
            coord: value.1,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

#[derive(Debug, Clone)]
pub struct ClumsyCrucible {
    grid: Grid<Block>
}

impl ClumsyCrucible {
    fn dijkstra(&self) -> usize {
        let mut visited = Grid::new(self.grid.n, self.grid.m, false);
        let mut acc = Grid::new(self.grid.n, self.grid.m, usize::MAX);
        let mut q: BinaryHeap<Node> = BinaryHeap::default();

        let start = (0, 0).into();
        let end = (self.grid.n-1, self.grid.m-1).into();
        q.push((0, start).into());
        acc[start] = 0;

        while let Some(node) = q.pop() {
            let coord = node.coord;
            if coord == end {
                return acc[coord];
            }

            if visited[coord] {
                continue;
            }

            visited[coord] = true;

            for neighbour in coord.cardinal_neighbours() {
                if !self.grid.is_in_bounds(neighbour) {
                    continue;
                }

                if self.grid[neighbour] as isize - self.grid[coord] as isize > 1 {
                    continue;
                }

                if acc[coord] + 1 < acc[neighbour] {
                    acc[neighbour] = acc[coord] + 1;
                    q.push((acc[neighbour], neighbour).into())
                }
            }
        }

        acc[self.end]
    }
}

impl FromStr for ClumsyCrucible {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { grid: Grid::from_str(s)? })
    }
}

impl Problem for ClumsyCrucible {
    const DAY: usize = 17;
    const TITLE: &'static str = "clumsy crucible";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(0)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(0)
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
        let solution = ClumsyCrucible::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(0, 0));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = ClumsyCrucible::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(102, 0));
    }
}
