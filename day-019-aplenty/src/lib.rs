use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, bail};
use aoc_common::interval::Interval;
use aoc_plumbing::Problem;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
enum Attribute {
    X,
    M,
    A,
    S,
}

impl FromStr for Attribute {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => bail!("invalid attribute"),
        })
    }
}

#[derive(Debug, Clone)]
struct Part {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}

impl Part {
    fn get(&self, attribute: &Attribute) -> isize {
        match attribute {
            Attribute::X => self.x,
            Attribute::M => self.m,
            Attribute::A => self.a,
            Attribute::S => self.s,
        }
    }

    fn rating(&self) -> isize {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s[1..s.len() - 1].split(',');

        let x = tokens
            .next()
            .and_then(|t| t[2..].parse().ok())
            .ok_or_else(|| anyhow!("invalid part"))?;
        let m = tokens
            .next()
            .and_then(|t| t[2..].parse().ok())
            .ok_or_else(|| anyhow!("invalid part"))?;
        let a = tokens
            .next()
            .and_then(|t| t[2..].parse().ok())
            .ok_or_else(|| anyhow!("invalid part"))?;
        let s = tokens
            .next()
            .and_then(|t| t[2..].parse().ok())
            .ok_or_else(|| anyhow!("invalid part"))?;

        Ok(Self { x, m, a, s })
    }
}

#[derive(Debug, Clone, Default)]
struct Ratings {
    x: Interval,
    m: Interval,
    a: Interval,
    s: Interval,
}

impl Ratings {
    fn new(min: isize, max: isize) -> Self {
        Self {
            x: Interval::new(min, max + 1),
            m: Interval::new(min, max + 1),
            a: Interval::new(min, max + 1),
            s: Interval::new(min, max + 1),
        }
    }

    fn get(&self, attribute: &Attribute) -> &Interval {
        match attribute {
            Attribute::X => &self.x,
            Attribute::M => &self.m,
            Attribute::A => &self.a,
            Attribute::S => &self.s,
        }
    }

    fn set(&mut self, attribute: &Attribute, interval: Interval) {
        match attribute {
            Attribute::X => self.x = interval,
            Attribute::M => self.m = interval,
            Attribute::A => self.a = interval,
            Attribute::S => self.s = interval,
        }
    }

    fn combinations(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    Reject,
    Accept,
    Workflow(String),
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Self::Reject,
            "A" => Self::Accept,
            _ => Self::Workflow(s.to_owned()),
        })
    }
}

#[derive(Debug, Clone)]
enum Condition {
    LessThan(Attribute, isize),
    GreaterThan(Attribute, isize),
}

impl Condition {
    fn apply(&self, part: &Part) -> bool {
        match self {
            Self::LessThan(a, x) => part.get(a) < *x,
            Self::GreaterThan(a, x) => part.get(a) > *x,
        }
    }

    fn apply_ratings(&self, ratings: Ratings) -> (Option<Ratings>, Option<Ratings>) {
        match self {
            Self::LessThan(a, x) => {
                if ratings.get(a).less_than(*x) {
                    (Some(ratings), None)
                } else if ratings.get(a).greater_than(*x) {
                    (None, Some(ratings))
                } else {
                    // safe to unwrap because we know interval contains x
                    let (left, right) = ratings.get(a).split(*x).unwrap();
                    if left.is_empty() {
                        (None, Some(ratings))
                    } else {
                        let mut matched = ratings.clone();
                        let mut unmatched = ratings;
                        matched.set(a, left);
                        unmatched.set(a, right);
                        (Some(matched), Some(unmatched))
                    }
                }
            }
            Self::GreaterThan(a, x) => {
                if ratings.get(a).less_than(*x) {
                    (None, Some(ratings))
                } else if ratings.get(a).greater_than(*x) {
                    (Some(ratings), None)
                } else {
                    // safe to unwrap because we know interval contains x
                    let (left, right) = ratings.get(a).split(*x + 1).unwrap();
                    if left.is_empty() {
                        (Some(ratings), None)
                    } else {
                        let mut matched = ratings.clone();
                        let mut unmatched = ratings;
                        matched.set(a, right);
                        unmatched.set(a, left);
                        (Some(matched), Some(unmatched))
                    }
                }
            }
        }
    }
}

impl FromStr for Condition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, n)) = s.split_once('>') {
            Ok(Self::GreaterThan(Attribute::from_str(a)?, n.parse()?))
        } else if let Some((a, n)) = s.split_once('<') {
            Ok(Self::LessThan(Attribute::from_str(a)?, n.parse()?))
        } else {
            bail!("invalid condition")
        }
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Conditional(Condition, Action),
    Unconditional(Action),
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<Action> {
        match self {
            Self::Conditional(c, a) => {
                if c.apply(part) {
                    Some(a.clone())
                } else {
                    None
                }
            }
            Self::Unconditional(a) => Some(a.clone()),
        }
    }

    fn apply_ratings(&self, ratings: Ratings) -> (Option<Ratings>, Option<Ratings>, Action) {
        match self {
            Self::Conditional(c, a) => {
                let (matched, unmatched) = c.apply_ratings(ratings);
                (matched, unmatched, a.clone())
            }
            Self::Unconditional(a) => (Some(ratings), None, a.clone()),
        }
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once(':') {
            Ok(Self::Conditional(
                Condition::from_str(a)?,
                Action::from_str(b)?,
            ))
        } else {
            Ok(Self::Unconditional(Action::from_str(s)?))
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn apply(&self, part: &Part) -> Action {
        // we unwrap because it is assumed that the last rule is always unconditional
        self.rules
            .iter()
            .find_map(|x| x.apply(part))
            .expect("invalid workflow, part did not match any rule")
    }

    fn apply_ratings(&self, ratings: Ratings) -> Vec<(Ratings, Action)> {
        let mut ret = Vec::default();
        let mut cur = ratings;

        for rule in &self.rules {
            let (matched, unmatched, action) = rule.apply_ratings(cur);

            if let Some(i) = matched {
                ret.push((i, action));
            }

            if unmatched.is_none() {
                break;
            }

            cur = unmatched.unwrap();
        }

        ret
    }
}

impl FromStr for Workflow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s[0..s.len() - 1].split_once('{') {
            let name = a.to_owned();
            let rules = b
                .split(',')
                .map(Rule::from_str)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Self { name, rules })
        } else {
            bail!("invalid workflow")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Aplenty {
    workflows: FxHashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Aplenty {
    fn sort(&self) -> isize {
        let mut ret = 0;

        for part in &self.parts {
            let mut cur = &self.workflows["in"];

            loop {
                match cur.apply(part) {
                    Action::Workflow(label) => cur = &self.workflows[&label],
                    Action::Reject => break,
                    Action::Accept => {
                        ret += part.rating();
                        break;
                    }
                }
            }
        }

        ret
    }

    fn combinations(&self) -> usize {
        let mut ret = 0;
        let mut q = VecDeque::default();
        q.push_back((Ratings::new(1, 4000), Action::Workflow("in".to_owned())));

        while let Some((ratings, action)) = q.pop_front() {
            match action {
                Action::Reject => (),
                Action::Accept => ret += ratings.combinations(),
                Action::Workflow(label) => q.extend(self.workflows[&label].apply_ratings(ratings)),
            }
        }

        ret
    }
}

impl FromStr for Aplenty {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once("\n\n") {
            let list = a
                .lines()
                .map(Workflow::from_str)
                .collect::<Result<Vec<_>, _>>()?;

            let mut workflows = FxHashMap::default();
            for w in list {
                workflows.insert(w.name.clone(), w);
            }

            let parts = b
                .lines()
                .map(Part::from_str)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Self { workflows, parts })
        } else {
            bail!("invalid input")
        }
    }
}

impl Problem for Aplenty {
    const DAY: usize = 19;
    const TITLE: &'static str = "aplenty";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = isize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.sort())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.combinations())
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
        let solution = Aplenty::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(446935, 141882534122898));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = Aplenty::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(19114, 167409079868000));
    }
}
