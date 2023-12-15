use std::str::FromStr;

use anyhow::Result;
use aoc_plumbing::Problem;

fn hash(s: &str) -> u8 {
    s.bytes().fold(0, |a, e| a.wrapping_add(e).wrapping_mul(17))
}

fn handle(token: &str, hashmap: &mut [Vec<(String, u8)>]) -> Result<()> {
    if token.ends_with('-') {
        let label = &token[0..token.len() - 1];
        let key = hash(label);
        let bucket = &mut hashmap[key as usize];
        let index = (0..bucket.len()).find(|i| bucket[*i].0 == label);

        if let Some(i) = index {
            bucket.remove(i);
        }
    } else {
        let label = &token[0..token.len() - 2];
        let key = hash(label);
        let lens = token[token.len() - 1..].parse::<u8>()?;
        let bucket = &mut hashmap[key as usize];
        let index = (0..bucket.len()).find(|i| bucket[*i].0 == label);

        if let Some(i) = index {
            bucket[i].1 = lens;
        } else {
            bucket.push((label.to_owned(), lens));
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct LensLibrary {
    part_one: usize,
    part_two: usize,
}

impl FromStr for LensLibrary {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.trim().split(',').collect();
        let part_one = tokens.iter().map(|x| hash(x) as usize).sum();

        let mut hashmap = vec![Vec::default(); 256];
        for token in tokens {
            handle(token, &mut hashmap)?;
        }

        let mut part_two = 0;
        for (i, bucket) in hashmap.iter().enumerate() {
            part_two += bucket
                .iter()
                .enumerate()
                .map(|(j, &(_, lens))| (i + 1) * (j + 1) * lens as usize)
                .sum::<usize>();
        }

        Ok(Self { part_one, part_two })
    }
}

impl Problem for LensLibrary {
    const DAY: usize = 15;
    const TITLE: &'static str = "lens library";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.part_one)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.part_two)
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
        let solution = LensLibrary::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(506891, 230462));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = LensLibrary::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1320, 145));
    }
}
