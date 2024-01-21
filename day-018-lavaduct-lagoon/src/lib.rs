use std::str::FromStr;

use anyhow::bail;
use aoc_common::{direction::Cardinal, grid::Coordinate};
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
struct Plan {
    dir: Cardinal,
    length: usize,
    hex_dir: Cardinal,
    hex_length: usize,
}

impl FromStr for Plan {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();

        let dir = match tokens.next() {
            Some("U") => Cardinal::North,
            Some("D") => Cardinal::South,
            Some("L") => Cardinal::West,
            Some("R") => Cardinal::East,
            _ => bail!("invalid plan"),
        };

        let length = if let Some(x) = tokens.next() {
            x.parse()?
        } else {
            bail!("invalid plan")
        };

        if let Some(x) = tokens.next() {
            let hex_length = usize::from_str_radix(&x[2..7], 16)?;
            let hex_dir = match x.as_bytes()[7] {
                b'0' => Cardinal::East,
                b'1' => Cardinal::South,
                b'2' => Cardinal::West,
                b'3' => Cardinal::North,
                _ => bail!("invalid plan"),
            };

            Ok(Self {
                dir,
                length,
                hex_dir,
                hex_length,
            })
        } else {
            bail!("invalid plan")
        }
    }
}

#[derive(Debug, Clone)]
pub struct LavaductLagoon {
    plans: Vec<Plan>,
}

impl LavaductLagoon {
    fn hex_area(&self) -> usize {
        let mut prev_point = Coordinate::from((0_isize, 0_isize));
        let mut prev_dir = self.plans[self.plans.len() - 1].hex_dir;
        let mut area = 0;
        let mut perimeter = 0;
        let mut left_turns = 0;
        let mut right_turns = 0;

        for plan in &self.plans {
            if prev_dir.right() == plan.hex_dir {
                right_turns += 1;
            } else if prev_dir.left() == plan.hex_dir {
                left_turns += 1;
            } else {
                // first plan and last plan form a straight side, not a corner
                // in this case, we add 1 more to the perimeter
                perimeter += 1;
            }

            let p = prev_point.steps(&plan.hex_dir, plan.hex_length);
            area += prev_point.x() * p.y() - prev_point.y() * p.x();
            perimeter += plan.hex_length - 1;

            prev_dir = plan.hex_dir;
            prev_point = p;
        }

        // positive means counterclockwise winding, negative means clockwise winding
        if area > 0 {
            (area as usize * 2 + perimeter * 2 + left_turns * 3 + right_turns) / 4
        } else {
            (-area as usize * 2 + perimeter * 2 + right_turns * 3 + left_turns) / 4
        }
    }

    fn area(&self) -> usize {
        let mut prev_point = Coordinate::from((0_isize, 0_isize));
        let mut prev_dir = self.plans[self.plans.len() - 1].dir;
        let mut area = 0;
        let mut perimeter = 0;
        let mut left_turns = 0;
        let mut right_turns = 0;

        for plan in &self.plans {
            if prev_dir.right() == plan.dir {
                right_turns += 1;
            } else if prev_dir.left() == plan.dir {
                left_turns += 1;
            } else {
                // first plan and last plan form a straight side, not a corner
                // in this case, we add 1 more to the perimeter
                perimeter += 1;
            }

            let p = prev_point.steps(&plan.dir, plan.length);
            area += prev_point.x() * p.y() - prev_point.y() * p.x();
            perimeter += plan.length - 1;

            prev_dir = plan.dir;
            prev_point = p;
        }

        // positive means counterclockwise winding, negative means clockwise winding
        if area > 0 {
            (area as usize * 2 + perimeter * 2 + left_turns * 3 + right_turns) / 4
        } else {
            (-area as usize * 2 + perimeter * 2 + right_turns * 3 + left_turns) / 4
        }
    }
}

impl FromStr for LavaductLagoon {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let plans = s
            .lines()
            .map(Plan::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { plans })
    }
}

impl Problem for LavaductLagoon {
    const DAY: usize = 18;
    const TITLE: &'static str = "lavaduct lagoon";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.area())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.hex_area())
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
        let solution = LavaductLagoon::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(50603, 96556251590677));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = LavaductLagoon::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(62, 952408144115));
    }
}
