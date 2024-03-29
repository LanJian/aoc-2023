use std::{collections::BinaryHeap, hash::Hash, str::FromStr};

use anyhow::anyhow;
use aoc_common::{
    direction::Cardinal,
    grid::{Coordinate, Grid},
};
use aoc_plumbing::Problem;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct MemoNode {
    orientation: Orientation,
    coord: Coordinate,
}

impl From<Node> for MemoNode {
    fn from(value: Node) -> Self {
        Self {
            orientation: value.orientation,
            coord: value.coord,
        }
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
        Self {
            dist,
            orientation,
            coord,
        }
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
    fn generate_neighbours_helper(
        &self,
        node: &Node,
        min: usize,
        max: usize,
        dir: &Cardinal,
        acc: &mut FxHashMap<MemoNode, usize>,
        q: &mut BinaryHeap<Node>,
    ) {
        let orientation = node.orientation.opposite();
        let mut dist = node.dist;

        for i in 1..=max {
            let coord = node.coord.steps(dir, i);

            if !self.grid.is_in_bounds(coord) {
                break;
            }

            dist += self.grid[coord].value;

            if i < min {
                continue;
            }

            let neighbour = Node::new(dist, orientation, coord);
            let neighbour_memo = neighbour.into();

            if dist < acc.get(&neighbour_memo).copied().unwrap_or(usize::MAX) {
                acc.insert(neighbour_memo, neighbour.dist);
                q.push(neighbour);
            }
        }
    }

    fn generate_neighbours(
        &self,
        node: &Node,
        min: usize,
        max: usize,
        acc: &mut FxHashMap<MemoNode, usize>,
        q: &mut BinaryHeap<Node>,
    ) {
        if node.orientation == Orientation::Horizontal {
            self.generate_neighbours_helper(node, min, max, &Cardinal::North, acc, q);
            self.generate_neighbours_helper(node, min, max, &Cardinal::South, acc, q);
        } else {
            self.generate_neighbours_helper(node, min, max, &Cardinal::East, acc, q);
            self.generate_neighbours_helper(node, min, max, &Cardinal::West, acc, q);
        }
    }

    fn dijkstra(&self, min: usize, max: usize) -> usize {
        let mut acc: FxHashMap<MemoNode, usize> = FxHashMap::default();
        let mut q: BinaryHeap<Node> = BinaryHeap::default();

        let start = (0_isize, 0_isize).into();
        let end = (self.grid.n - 1, self.grid.m - 1).into();

        let node1 = Node::new(0, Orientation::Horizontal, start);
        let node2 = Node::new(0, Orientation::Vertical, start);
        acc.insert(node1.into(), node1.dist);
        acc.insert(node2.into(), node2.dist);
        q.push(node1);
        q.push(node2);

        while let Some(node) = q.pop() {
            let coord = node.coord;
            if coord == end {
                return node.dist;
            }

            if acc.get(&node.into()).copied().unwrap_or(usize::MAX) < node.dist {
                continue;
            }

            self.generate_neighbours(&node, min, max, &mut acc, &mut q);
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
        Ok(self.dijkstra(1, 3))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.dijkstra(4, 10))
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
