use std::str::FromStr;

use anyhow::bail;
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn ways_to_beat_record(&self) -> usize {
        // solve x^2 - bx + c = 0 using quadratic formula
        // where b is the time and c is the distance
        let (a, b, c) = (1.0, -(self.time as f64), self.distance as f64);

        let sqrt_d = (b * b - 4.0 * a * c).sqrt();
        let roots = ((-b + sqrt_d) / (2.0 * a), (-b - sqrt_d) / (2.0 * a));

        let min = roots.0.min(roots.1).floor() as usize;
        self.time - min - min - 1
    }
}

#[derive(Debug, Clone)]
pub struct WaitForIt {
    races: Vec<Race>,
}

impl WaitForIt {
    fn margin_of_error(&self) -> usize {
        self.races.iter().map(|x| x.ways_to_beat_record()).product()
    }

    fn margin_of_error_single_race(&self) -> usize {
        let (time, distance) = self.races.iter().fold((0, 0), |a, e| {
            (
                a.0 * 10_u32.pow(e.time.checked_ilog10().unwrap() + 1) as usize + e.time,
                a.1 * 10_u32.pow(e.distance.checked_ilog10().unwrap() + 1) as usize + e.distance,
            )
        });

        let race = Race { time, distance };
        race.ways_to_beat_record()
    }
}

impl FromStr for WaitForIt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut races = Vec::default();
        let mut iter = s.lines();

        if let Some(l) = iter.next() {
            for token in l.split_whitespace().skip(1) {
                races.push(Race {
                    time: token.parse()?,
                    distance: 0,
                });
            }
        } else {
            bail!("invalid number of lines in input");
        }

        if let Some(l) = iter.next() {
            for (i, token) in l.split_whitespace().skip(1).enumerate() {
                races[i].distance = token.parse()?;
            }
        } else {
            bail!("invalid number of lines in input");
        }

        Ok(Self { races })
    }
}

impl Problem for WaitForIt {
    const DAY: usize = 6;
    const TITLE: &'static str = "wait for it";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.margin_of_error())
    }
    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.margin_of_error_single_race())
        //Ok(self.foo())
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
        let solution = WaitForIt::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(114400, 21039729));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = WaitForIt::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(288, 71503));
    }
}
