use rayon::prelude::*;
use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, bail};
use aoc_common::{
    direction::Cardinal,
    grid::{Coordinate, Grid},
};
use aoc_plumbing::Problem;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Slope(Cardinal),
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '^' => Self::Slope(Cardinal::North),
            '>' => Self::Slope(Cardinal::East),
            'v' => Self::Slope(Cardinal::South),
            '<' => Self::Slope(Cardinal::West),
            _ => bail!("invalid tile"),
        })
    }
}

#[derive(Debug, Clone)]
struct Node {
    idx: usize,
    coord: Coordinate,
    neighbours: Vec<(usize, usize)>,
}

impl Node {
    fn new(idx: usize, coord: Coordinate) -> Self {
        Self {
            idx,
            coord,
            neighbours: Vec::default(),
        }
    }
}

fn is_visited(idx: usize, visited: u64) -> bool {
    1u64 << idx & visited > 0
}

fn visit(idx: usize, visited: u64) -> u64 {
    visited | 1u64 << idx
}

type Graph = Vec<Node>;

#[derive(Debug, Clone)]
pub struct ALongWalk {
    grid: Grid<Tile>,
}

impl ALongWalk {
    fn find_vertices(&self) -> Graph {
        let n = self.grid.n;
        let m = self.grid.m;
        let mut graph = Vec::default();
        graph.push(Node::new(0, Coordinate::new(0, 1)));
        graph.push(Node::new(1, (n - 1, m - 2).into()));

        for i in 1..n - 1 {
            for j in 1..m - 1 {
                let coord = (i, j).into();
                let tile = self.grid[coord];

                if tile == Tile::Wall {
                    continue;
                }

                if coord
                    .cardinal_neighbours()
                    .iter()
                    .filter(|&n| self.grid.is_in_bounds(*n) && self.grid[*n] != Tile::Wall)
                    .count()
                    > 2
                {
                    graph.push(Node::new(graph.len(), coord));
                }
            }
        }

        graph
    }

    fn build_graph(&self) -> Graph {
        let n = self.grid.n;
        let m = self.grid.m;
        let mut graph = self.find_vertices();
        let mut visited = Grid::new(n, m, false);
        let mut q = VecDeque::default();

        let coords_to_ids = FxHashMap::from_iter(graph.iter().map(|x| (x.coord, x.idx)));

        for u in 0..graph.len() {
            let node = &graph[u];
            q.clear();
            q.push_back((node.coord, 0));

            while let Some((coord, dist)) = q.pop_front() {
                if let Some(&v) = coords_to_ids.get(&coord) {
                    if dist > 0 {
                        graph[u].neighbours.push((v, dist));
                        graph[v].neighbours.push((u, dist));
                        continue;
                    }
                }

                visited[coord] = true;

                for n in coord.cardinal_neighbours() {
                    if self.grid.is_in_bounds(n) && self.grid[n] != Tile::Wall && !visited[n] {
                        q.push_back((n, dist + 1))
                    }
                }
            }
        }

        graph
    }

    fn longest_path_flat(&self, start_depth: usize) -> Option<usize> {
        let graph = self.build_graph();
        let (penultimate, last_cost) = graph[1].neighbours[0];

        let mut cur = vec![(0usize, 0usize, 0usize, 0u64)];
        let mut next = Vec::default();

        for _ in 0..start_depth {
            next.extend(cur.drain(..).flat_map(|(u, cost, depth, visited)| {
                graph[u]
                    .neighbours
                    .iter()
                    .filter(move |&(v, _)| !is_visited(*v, visited))
                    .map(move |&(v, c)| (v, cost + c, depth + 1, visit(u, visited)))
            }));

            std::mem::swap(&mut cur, &mut next);
        }

        cur.into_par_iter()
            .filter_map(|(u, cost, _, visited)| {
                Self::longest_path_flat_helper(u, penultimate, &graph, visited)
                    .map(|x| x + last_cost + cost)
            })
            .max()
    }

    fn longest_path_flat_helper(
        start: usize,
        end: usize,
        graph: &Graph,
        visited: u64,
    ) -> Option<usize> {
        if start == end {
            return Some(0);
        }

        if is_visited(start, visited) {
            return None;
        }

        let new_visited = visit(start, visited);
        let result = graph[start]
            .neighbours
            .iter()
            .filter_map(|&(vertex, cost)| {
                Self::longest_path_flat_helper(vertex, end, graph, new_visited).map(|x| x + cost)
            })
            .max();

        result
    }

    fn longest_path(
        &self,
        start: Coordinate,
        end: Coordinate,
        visited: &mut Grid<bool>,
    ) -> Option<usize> {
        if start == end {
            return Some(0);
        }

        if !self.grid.is_in_bounds(start) {
            return None;
        }

        if visited[start] {
            return None;
        }

        visited[start] = true;

        let tile = self.grid[start];
        let result = match tile {
            Tile::Slope(d) => self.longest_path(start.neighbour(&d), end, visited),
            Tile::Empty => start
                .cardinal_neighbours()
                .iter()
                .filter_map(|x| self.longest_path(*x, end, visited))
                .max(),
            Tile::Wall => None,
        };

        visited[start] = false;

        result.map(|x| x + 1)
    }
}

impl FromStr for ALongWalk {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        Ok(Self { grid })
    }
}

impl Problem for ALongWalk {
    const DAY: usize = 23;
    const TITLE: &'static str = "a long walk";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.longest_path(
            (0isize, 1isize).into(),
            (self.grid.n - 1, self.grid.m - 2).into(),
            &mut Grid::new(self.grid.n, self.grid.m, false),
        )
        .ok_or_else(|| anyhow!("no path found"))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.longest_path_flat(10)
            .ok_or_else(|| anyhow!("no path found"))
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
        let solution = ALongWalk::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(2250, 6470));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let mut instance = ALongWalk::instance(&input).unwrap();
        assert_eq!(instance.part_one().unwrap(), 94);
        assert_eq!(instance.longest_path_flat(3).unwrap(), 154);
    }
}
