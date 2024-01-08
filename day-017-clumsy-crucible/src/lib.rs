use std::{collections::BinaryHeap, str::FromStr};

use anyhow::anyhow;
use aoc_common::{
    direction::Cardinal,
    grid::{Coordinate, Grid},
};
use aoc_plumbing::Problem;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    fn opposite(&self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Block {
    value: usize,
}

impl TryFrom<char> for Block {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Self {
            value: value
                .to_digit(10)
                .map(|x| x as usize)
                .ok_or_else(|| anyhow!("invalid block value"))?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    dist: usize,
    orientation: Orientation,
    coord: Coordinate,
}

impl Node {
    pub fn new(dist: usize, orientation: Orientation, coord: Coordinate) -> Self {
        Self { dist, orientation, coord }
    }

    fn key(&self) -> u32 {
        (self.coord.row() as u32) << 24
            | (self.coord.col() as u32) << 16
            | (self.orientation as u32)
    }

    fn neighbours_helper(
        &self,
        min: usize,
        max: usize,
        dir: &Cardinal,
        grid: &Grid<Block>,
        ret: &mut Vec<Self>,
    ) {
        let orientation = self.orientation.opposite();
        let mut dist = self.dist;

        for i in 1..=max {
            let coord = self.coord.steps(dir, i);

            if !grid.is_in_bounds(coord) {
                break;
            }

            dist += grid[coord].value;

            if i < min {
                continue;
            }

            ret.push(Self::new(dist, orientation, coord));
        }
    }

    fn neighbours(&self, min: usize, max: usize, grid: &Grid<Block>) -> Vec<Self> {
        let mut ret = Vec::default();

        if self.orientation == Orientation::Horizontal {
            self.neighbours_helper(min, max, &Cardinal::North, grid, &mut ret);
            self.neighbours_helper(min, max, &Cardinal::South, grid, &mut ret);
        } else {
            self.neighbours_helper(min, max, &Cardinal::East, grid, &mut ret);
            self.neighbours_helper(min, max, &Cardinal::West, grid, &mut ret);
        }

        ret
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

#[derive(Debug, Clone)]
pub struct ClumsyCrucible {
    grid: Grid<Block>,
}

impl ClumsyCrucible {
    fn dijkstra_two(&self, min: usize, max: usize) -> usize {
        let mut visited = FxHashSet::default();
        let mut acc = FxHashMap::default();
        let mut q: BinaryHeap<Node> = BinaryHeap::default();

        let start = (0_isize, 0_isize).into();
        let end = (self.grid.n - 1, self.grid.m - 1).into();

        let node1 = Node::new(0, Orientation::Horizontal, start);
        let node2 = Node::new(0, Orientation::Vertical, start);
        acc.insert(node1.key(), node1.dist);
        acc.insert(node2.key(), node2.dist);
        q.push(node1);
        q.push(node2);

        while let Some(node) = q.pop() {
            let coord = node.coord;
            if coord == end {
                return node.dist;
            }

            if visited.contains(&node.key()) {
                continue;
            }

            visited.insert(node.key());

            for neighbour in &node.neighbours(min, max, &self.grid) {
                if neighbour.dist < *acc.get(&neighbour.key()).unwrap_or(&usize::MAX) {
                    acc.insert(neighbour.key(), neighbour.dist);
                    q.push(*neighbour);
                }
            }
        }

        unreachable!()
    }
}

impl FromStr for ClumsyCrucible {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: Grid::from_str(s)?,
        })
    }
}

impl Problem for ClumsyCrucible {
    const DAY: usize = 17;
    const TITLE: &'static str = "clumsy crucible";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.dijkstra_two(1, 3))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.dijkstra_two(4, 10))
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
        assert_eq!(solution, Solution::new(1099, 1266));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = ClumsyCrucible::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(102, 94));
    }
}
