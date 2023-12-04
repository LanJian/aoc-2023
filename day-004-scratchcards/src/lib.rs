use std::str::FromStr;

use anyhow::{bail, Result};
use aoc_plumbing::Problem;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone)]
struct Card {
    matching_count: usize,
    points: u32,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((_, right)) = s.split_once(": ") {
            if let Some((winning_token, mine_token)) = right.split_once(" | ") {
                let winning_numbers = winning_token
                    .split_whitespace()
                    .map(|x| x.parse::<u32>())
                    .collect::<Result<FxHashSet<u32>, _>>()?;
                let my_numbers = mine_token
                    .split_whitespace()
                    .map(|x| x.parse::<u32>())
                    .collect::<Result<FxHashSet<u32>, _>>()?;
                let matching_count = winning_numbers.intersection(&my_numbers).count();
                let points = match matching_count {
                    0 => 0,
                    _ => 2_u32.pow((matching_count - 1) as u32),
                };

                return Ok(Self {
                    matching_count,
                    points,
                });
            }
        }

        bail!("could not parse card input")
    }
}

#[derive(Debug, Clone)]
pub struct Scratchcards {
    cards: Vec<Card>,
}

impl FromStr for Scratchcards {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .lines()
            .map(Card::from_str)
            .collect::<Result<Vec<Card>>>()?;

        Ok(Self { cards })
    }
}

impl Scratchcards {
    fn total_points(&self) -> u32 {
        self.cards.iter().map(|x| x.points).sum()
    }

    fn total_copies(&self) -> u32 {
        let mut copies = vec![1; self.cards.len()];
        let mut count = 0;

        for i in 0..self.cards.len() {
            let card = &self.cards[i];

            for j in (i + 1)..(i + card.matching_count + 1) {
                copies[j] += copies[i];
            }

            count += copies[i];
        }

        count
    }
}

impl Problem for Scratchcards {
    const DAY: usize = 4;
    const TITLE: &'static str = "scratchcards";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u32;
    type P2 = u32;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.total_points())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.total_copies())
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
        let solution = Scratchcards::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(17803, 5554894));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = Scratchcards::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(13, 30));
    }
}
