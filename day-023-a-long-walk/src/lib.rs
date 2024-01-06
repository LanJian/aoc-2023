use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, bail};
use aoc_common::{
    direction::CardinalDirection,
    grid::{Coordinate, Grid},
};
use aoc_plumbing::Problem;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Slope(CardinalDirection),
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '^' => Self::Slope(CardinalDirection::North),
            '>' => Self::Slope(CardinalDirection::East),
            'v' => Self::Slope(CardinalDirection::South),
            '<' => Self::Slope(CardinalDirection::West),
            _ => bail!("invalid tile"),
        })
    }
}

type Graph = FxHashMap<Coordinate, FxHashSet<(Coordinate, usize)>>;

#[derive(Debug, Clone)]
pub struct ALongWalk {
    grid: Grid<Tile>,
}

impl ALongWalk {
    fn find_vertices(&self) -> Graph {
        let n = self.grid.n;
        let m = self.grid.m;
        let mut graph = FxHashMap::default();
        graph.insert(Coordinate::new(0, 1), FxHashSet::default());
        graph.insert((n - 1, m - 2).into(), FxHashSet::default());

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
                    graph.insert(coord, FxHashSet::default());
                }
            }
        }

        graph
    }

    fn build_graph(&self) -> Graph {
        let n = self.grid.n;
        let m = self.grid.m;
        let mut graph = self.find_vertices();
        let vertices: Vec<_> = graph.keys().cloned().collect();
        let mut visited = Grid::new(n, m, false);
        let mut q = VecDeque::default();

        for vertex in vertices {
            q.clear();
            q.push_back((vertex, 0));

            while let Some((coord, dist)) = q.pop_front() {
                if dist > 0 && graph.contains_key(&coord) {
                    graph.entry(vertex).and_modify(|x| {
                        x.insert((coord, dist));
                    });
                    graph.entry(coord).and_modify(|x| {
                        x.insert((vertex, dist));
                    });
                    continue;
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

    fn longest_path_flat(&self) -> Option<usize> {
        let graph = self.build_graph();
        Self::longest_path_flat_helper(
            (0isize, 1isize).into(),
            (self.grid.n - 1, self.grid.m - 2).into(),
            &graph,
            &mut FxHashSet::default(),
        )
    }

    fn longest_path_flat_helper(
        start: Coordinate,
        end: Coordinate,
        graph: &Graph,
        visited: &mut FxHashSet<Coordinate>,
    ) -> Option<usize> {
        if start == end {
            return Some(0);
        }

        if visited.contains(&start) {
            return None;
        }

        visited.insert(start);

        let result = graph[&start]
            .iter()
            .filter_map(|&(vertex, cost)| {
                Self::longest_path_flat_helper(vertex, end, graph, visited).map(|x| x + cost)
            })
            .max();

        visited.remove(&start);

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
        self.longest_path_flat()
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
        let solution = ALongWalk::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(94, 154));
    }
}
