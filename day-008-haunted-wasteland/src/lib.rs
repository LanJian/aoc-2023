use anyhow::{anyhow, bail};
use aoc_plumbing::Problem;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::str::FromStr;

fn label_to_id(label: &str) -> u32 {
    label.bytes().fold(0, |a, c| a << 8 | c as u32)
}

fn ends_with(id: u32, letter: u8) -> bool {
    id as u8 == letter
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    left: u32,
    right: u32,
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s[1..s.len() - 1].split_once(", ") {
            Ok(Node {
                left: label_to_id(left),
                right: label_to_id(right),
            })
        } else {
            bail!("could not parse node")
        }
    }
}

#[derive(Debug, Clone)]
pub struct HauntedWasteland {
    directions: Vec<Direction>,
    graph: FxHashMap<u32, Node>,
}

impl HauntedWasteland {
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }

        Self::gcd(b, a % b)
    }

    fn lcm(a: usize, b: usize) -> usize {
        a * b / Self::gcd(a, b)
    }

    fn traverse_one(&self, source: u32, direction: &Direction) -> u32 {
        match direction {
            Direction::Left => self.graph[&source].left,
            Direction::Right => self.graph[&source].right,
        }
    }

    fn traverse(&self, source: u32, destination: u32) -> usize {
        let mut dist = 0;
        let mut dir_index = 0;
        let mut cur = source;

        while cur != destination {
            dir_index %= self.directions.len();
            let direction = &self.directions[dir_index];
            cur = self.traverse_one(cur, direction);

            dir_index += 1;
            dist += 1;
        }

        dist
    }

    fn traverse_to_any_z(&self, source: u32) -> usize {
        let mut dist = 0;
        let mut dir_index = 0;
        let mut cur = source;

        while !ends_with(cur, b'Z') {
            dir_index %= self.directions.len();
            let direction = &self.directions[dir_index];
            cur = self.traverse_one(cur, direction);

            dir_index += 1;
            dist += 1;
        }

        dist
    }
}

impl FromStr for HauntedWasteland {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        let directions: Vec<_> = iter
            .next()
            .ok_or_else(|| anyhow!("not enough lines in input"))?
            .chars()
            .map(Direction::from)
            .collect();

        iter.next();

        let mut graph = FxHashMap::default();
        for line in iter {
            if let Some((left, right)) = line.split_once(" = ") {
                graph.insert(label_to_id(left), Node::from_str(right)?);
            } else {
                bail!("could not parse graph")
            }
        }

        Ok(Self { directions, graph })
    }
}

impl Problem for HauntedWasteland {
    const DAY: usize = 8;
    const TITLE: &'static str = "haunted wasteland";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.traverse(label_to_id("AAA"), label_to_id("ZZZ")))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self
            .graph
            .keys()
            .filter(|&x| ends_with(*x, b'A'))
            .collect::<Vec<_>>()
            .par_iter()
            .map(|&&x| self.traverse_to_any_z(x))
            .reduce(|| 1, HauntedWasteland::lcm))
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
        let solution = HauntedWasteland::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(16897, 16563603485021));
    }

    #[test]
    fn ends_with_test() {
        assert!(ends_with(label_to_id("AAA"), b'A'));
        assert!(ends_with(label_to_id("AAZ"), b'Z'));
    }

    #[test]
    fn gcd_test() {
        assert_eq!(HauntedWasteland::gcd(48, 18), 6);
    }

    #[test]
    fn lcm_test() {
        assert_eq!(HauntedWasteland::lcm(21, 6), 42);
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = HauntedWasteland::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(2, 2));
    }

    #[test]
    fn example_two() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let solution = HauntedWasteland::solve(input).unwrap();
        assert_eq!(solution, Solution::new(6, 6));
    }

    #[test]
    fn example_part_two() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let mut instance = HauntedWasteland::instance(input).unwrap();
        assert_eq!(instance.part_two().unwrap(), 6);
    }
}
