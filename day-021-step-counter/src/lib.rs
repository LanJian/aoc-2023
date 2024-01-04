use std::{collections::VecDeque, str::FromStr};

use anyhow::bail;
use aoc_common::grid::{Coordinate, Grid};
use aoc_plumbing::Problem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Garden,
    Rock,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' | 'S' => Self::Garden,
            '#' => Self::Rock,
            _ => bail!("invalid tile"),
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Parity {
    Odd,
    Even,
}

#[derive(Debug, Clone)]
pub struct StepCounter {
    grid: Grid<Tile>,
}

impl StepCounter {
    fn step_counter(&self, steps: usize) -> usize {
        // we are making a bunch of assumptions here:
        // - all inputs have the same size and are square
        // - start is always in the middle
        // - the edges and the middle row and column does not have rocks
        //
        // this solution follows the logic behind this [diagram](https://raw.githubusercontent.com/Manitary/advent-of-code/c44838423066b3c8d446f0d94f2a19d675f2b6dc/2023/python/day21.png)
        let n = self.grid.n;
        let r = n / 2;
        let x = steps / n;
        let mut ret = 0;

        // add regions that are completely reachable
        let even_regions = x * x;
        let odd_regions = (x - 1) * (x - 1);
        ret += self.bfs(self.start(), steps, Parity::Even) * even_regions;
        ret += self.bfs(self.start(), steps, Parity::Odd) * odd_regions;

        // add the 4 cardinal regions
        //
        // note that the parity should be `Odd` but is `Even` here because we enter into the
        // region on an odd tile, which means we must flip the parity to even from the perspective
        // of the start tile.
        let parity = Parity::Even;
        ret += self.bfs((n - 1, r).into(), n - 1, parity); // s
        ret += self.bfs((0, r).into(), n - 1, parity); // n
        ret += self.bfs((r, 0).into(), n - 1, parity); // w
        ret += self.bfs((r, n - 1).into(), n - 1, parity); // e

        // add all the "sides" of the diamond
        let outer_parity = Parity::Even;
        let inner_parity = Parity::Odd;

        // ne
        ret += self.bfs((0, n - 1).into(), r - 1, outer_parity) * x;
        ret += self.bfs((0, n - 1).into(), n + r - 1, inner_parity) * (x - 1);

        // se
        ret += self.bfs((n - 1, n - 1).into(), r - 1, outer_parity) * x;
        ret += self.bfs((n - 1, n - 1).into(), n + r - 1, inner_parity) * (x - 1);

        // nw
        ret += self.bfs((0isize, 0isize).into(), r - 1, outer_parity) * x;
        ret += self.bfs((0isize, 0isize).into(), n + r - 1, inner_parity) * (x - 1);

        // sw
        ret += self.bfs((n - 1, 0).into(), r - 1, outer_parity) * x;
        ret += self.bfs((n - 1, 0).into(), n + r - 1, inner_parity) * (x - 1);

        ret
    }

    fn bfs(&self, start: Coordinate, steps: usize, parity: Parity) -> usize {
        let mut ret = 0;
        let mut visited = Grid::new(self.grid.n, self.grid.m, false);
        let mut q = VecDeque::default();
        q.push_back((start, 0));

        while let Some((coord, dist)) = q.pop_front() {
            if dist > steps
                || !self.grid.is_in_bounds(coord)
                || self.grid[coord] == Tile::Rock
                || visited[coord]
            {
                continue;
            }

            visited[coord] = true;

            match parity {
                Parity::Odd if dist % 2 == 1 => ret += 1,
                Parity::Even if dist % 2 == 0 => ret += 1,
                _ => (),
            }

            for n in coord.cardinal_neighbours() {
                q.push_back((n, dist + 1));
            }
        }

        ret
    }

    fn start(&self) -> Coordinate {
        let r = self.grid.n / 2;
        (r, r).into()
    }
}

impl FromStr for StepCounter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        Ok(Self { grid })
    }
}

impl Problem for StepCounter {
    const DAY: usize = 21;
    const TITLE: &'static str = "step counter";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.bfs(self.start(), 64, Parity::Even))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.step_counter(26501365))
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
        let solution = StepCounter::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(3677, 609585229256084));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let instance = StepCounter::instance(&input).unwrap();
        assert_eq!(instance.bfs(instance.start(), 6, Parity::Even), 16);
    }
}
