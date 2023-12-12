use std::str::FromStr;

use anyhow::bail;
use aoc_plumbing::Problem;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

impl Spring {
    fn potentially_damaged(&self) -> bool {
       *self == Self::Unknown  || *self == Self::Damaged
    }

    fn potentially_operational(&self) -> bool {
       *self == Self::Unknown  || *self == Self::Operational
    }

    fn damaged(&self) -> bool {
        *self == Self::Damaged
    }

    fn operational(&self) -> bool {
        *self == Self::Operational
    }
}

impl TryFrom<char> for Spring {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
       let ret = match value {
           '?' => Self::Unknown,
           '#' => Self::Damaged,
           '.' => Self::Operational,
            _ => bail!("unexpected spring character")
       };

        Ok(ret)
    }
}

#[derive(Debug, Clone)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl Record {
    fn print(springs: &[Spring]) {
        let foo: String = springs.iter().map(|x| match x {
            Spring::Unknown => '?',
            Spring::Damaged => '#',
            Spring::Operational => '.',
        }).collect();
        println!("{}", foo);
    }

    fn arrangements(&self) -> usize {
        let ret = self.arrangements_helper(&self.springs, &self.groups, Vec::default());
        println!("----------------------------------");
        Self::print(&self.springs);
        dbg!(&self.groups);
        dbg!(&ret);
        println!("====================================================================");
        ret
    }

    fn arrangements_helper(&self, springs: &[Spring], groups: &[usize], acc: Vec<Spring>) -> usize {
        if groups.is_empty() {
            let mut new_acc = acc.clone();
            new_acc.extend(springs);
            Self::print(&new_acc);

            return 1;
        }

        let group = groups[0];
        if group > springs.len() {
            return 0;
        }

        let mut ret = 0;

        // fill the group now
        if self.all_potentially_damaged(&springs[0..group]) {
            if group == springs.len() {
                // if this fills all the way to the end...
                if groups.len() == 1 {
                    // and there are no more groups to fill, then this is one arrangement
                    let mut new_acc = acc.clone();
                    new_acc.extend(vec![Spring::Damaged; group]);
                    Self::print(&new_acc);

                    return 1;
                } else {
                    // but there are more groups to fill, then it's impossible
                    return 0;
                }
            } else if springs[group].potentially_operational() {
                // we can fill this group here, so we recur starting from after the filled group
                // plus one buffer space
                let mut new_acc = acc.clone();
                new_acc.extend(vec![Spring::Damaged; group]);
                new_acc.push(Spring::Operational);
                ret += self.arrangements_helper(&springs[group+1..], &groups[1..], new_acc);
            }
            // otherwise we cannot fill the group here, so we will kick in down the line
        }

        // or kick it down the line
        if !springs[0].damaged() {
            // we can only kick it if the leading spring is not damaged. if it is damaged, then
            // we have to fill the group now
            let mut new_acc = acc.clone();
            new_acc.push(springs[0]);
            ret += self.arrangements_helper(&springs[1..], groups, new_acc);
        }

        ret
    }

    fn all_potentially_damaged(&self, springs: &[Spring]) -> bool {
        springs.iter().all(|x| x.potentially_damaged())
    }
}

impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s.split_once(' ') {
            let springs = left.chars()
                .map(|x| x.try_into())
                .collect::<Result<Vec<_>, _>>()?;

            let groups = right.split(',')
                .map(|x| x.parse())
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Self { springs, groups })
        } else {
            bail!("could not parse record")
        }
    }
}


#[derive(Debug, Clone)]
pub struct HotSprings {
    records: Vec<Record>
}

impl HotSprings {
    fn sum_arrangements(&self) -> usize {
        let ret = self.records.iter().map(|x| x.arrangements()).sum();
        dbg!(&self.records.len());
        ret
        //self.records.iter().rev().take(1).map(|x| x.arrangements()).sum()
    }
}

impl FromStr for HotSprings {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let records = s.lines().map(Record::from_str).collect::<Result<Vec<_>, _>>()?;
        Ok(Self { records })
    }
}

impl Problem for HotSprings {
    const DAY: usize = 12;
    const TITLE: &'static str = "hot springs";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.sum_arrangements())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(0)
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
        let solution = HotSprings::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(0, 0));
    }

    #[test]
    fn arrangements_test() {
        let mut record = Record::from_str("### 3").unwrap();
        assert_eq!(record.arrangements(), 1);

        record = Record::from_str("?. 1").unwrap();
        assert_eq!(record.arrangements(), 1);

        record = Record::from_str(".### 3").unwrap();
        assert_eq!(record.arrangements(), 1);

        record = Record::from_str("?.# 1,1").unwrap();
        assert_eq!(record.arrangements(), 1);

        record = Record::from_str("?...??#??. 1,5").unwrap();
        assert_eq!(record.arrangements(), 1);
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = HotSprings::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(21, 0));
    }
}
