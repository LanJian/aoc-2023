use std::{collections::BinaryHeap, str::FromStr};

use anyhow::anyhow;
use aoc_common::{
    direction::CardinalDirection,
    grid::{Coordinate, Grid},
};
use aoc_plumbing::Problem;
use rustc_hash::{FxHashMap, FxHashSet};

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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    dist: usize,
    moved: usize,
    dir: CardinalDirection,
    coord: Coordinate,
}

impl Node {
    fn key(&self) -> u32 {
        (self.coord.row() as u32) << 24
            | (self.coord.col() as u32) << 16
            | (self.moved as u32) << 8
            | (self.dir as u32)
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
    fn dijkstra_two(&self) -> usize {
        let mut visited = FxHashSet::default();
        let mut acc = FxHashMap::default();
        let mut q: BinaryHeap<Node> = BinaryHeap::default();

        let start = (0_isize, 0_isize).into();
        let end = (self.grid.n - 1, self.grid.m - 1).into();

        let node1 = Node {
            dist: 0,
            moved: 10,
            dir: CardinalDirection::South,
            coord: start,
        };
        let node2 = Node {
            dist: 0,
            moved: 10,
            dir: CardinalDirection::East,
            coord: start,
        };
        acc.insert(node1.key(), 0);
        acc.insert(node2.key(), 0);
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

            'outer: for d in CardinalDirection::all() {
                if node.dir.opposite() == d {
                    continue;
                }

                let next_dist;
                let next_moved;
                let next_coord;

                if node.dir == d {
                    if node.moved >= 10 {
                        continue;
                    }

                    next_coord = coord.neighbour(&d);
                    if !self.grid.is_in_bounds(next_coord) {
                        continue;
                    }

                    next_dist = node.dist + self.grid[next_coord].value;
                    next_moved = node.moved + 1;
                } else {
                    let mut cur_coord = coord;
                    let mut cur_dist = node.dist;
                    next_moved = 4;

                    for _ in 0..4 {
                        cur_coord = cur_coord.neighbour(&d);
                        if !self.grid.is_in_bounds(cur_coord) {
                            continue 'outer;
                        }

                        cur_dist += self.grid[cur_coord].value;
                    }

                    next_coord = cur_coord;
                    next_dist = cur_dist;
                }

                let next_node = Node {
                    dist: next_dist,
                    moved: next_moved,
                    dir: d,
                    coord: next_coord,
                };

                if next_dist < *acc.get(&next_node.key()).unwrap_or(&usize::MAX) {
                    acc.insert(next_node.key(), next_dist);
                    q.push(next_node);
                }
            }
        }

        unreachable!()
    }

    fn dijkstra_one(&self) -> usize {
        let mut visited = FxHashSet::default();
        let mut acc = FxHashMap::default();
        let mut q: BinaryHeap<Node> = BinaryHeap::default();

        let start = (0_isize, 0_isize).into();
        let end = (self.grid.n - 1, self.grid.m - 1).into();

        let start_node = Node {
            dist: 0,
            moved: 0,
            dir: CardinalDirection::South,
            coord: start,
        };
        acc.insert(start_node.key(), 0);
        q.push(start_node);

        while let Some(node) = q.pop() {
            let coord = node.coord;
            if coord == end {
                return node.dist;
            }

            if visited.contains(&node.key()) {
                continue;
            }

            visited.insert(node.key());

            for d in CardinalDirection::all() {
                if node.dir.opposite() == d {
                    continue;
                }

                let neighbour = coord.neighbour(&d);

                if !self.grid.is_in_bounds(neighbour) {
                    continue;
                }

                if node.dir == d && node.moved >= 3 {
                    continue;
                }

                let next_moved = if node.dir == d { node.moved + 1 } else { 1 };
                let next_dist = node.dist + self.grid[neighbour].value;
                let next_node = Node {
                    dist: next_dist,
                    moved: next_moved,
                    dir: d,
                    coord: neighbour,
                };
                if next_dist < *acc.get(&next_node.key()).unwrap_or(&usize::MAX) {
                    acc.insert(next_node.key(), next_dist);
                    q.push(next_node);
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
        Ok(self.dijkstra_one())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.dijkstra_two())
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
