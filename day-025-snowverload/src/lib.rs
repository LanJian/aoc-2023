use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, Result};
use aoc_plumbing::Problem;
use rand::{seq::SliceRandom, thread_rng};
use rustc_hash::{FxHashMap, FxHashSet};

type Graph = FxHashMap<u16, Vec<u16>>;

#[derive(Debug, Clone)]
pub struct Snowverload {
    graph: Graph,
    vertices: Vec<u16>,
}

impl Snowverload {
    fn min_cut(&self) -> Option<usize> {
        let mut rng = thread_rng();

        loop {
            // randomly choose source and sink until we find a pair where the max flow between
            // them is 3. the groups are reasonably evenly distributed so that we have around 50%
            // chance of choosing a correct pair
            let mut iter = self.vertices.choose_multiple(&mut rng, 2).copied();
            let (source, sink) = (iter.next().unwrap(), iter.next().unwrap());

            if let Some(result) = self.min_cut_helper(source, sink) {
                return Some(result);
            }
        }
    }

    fn min_cut_helper(&self, source: u16, sink: u16) -> Option<usize> {
        let mut pred = FxHashMap::default();
        let mut q = VecDeque::default();
        let mut visited_edges = FxHashSet::default();
        let mut flow = 0;

        // do bfs over and over again until we can't reach the sink anymore, or if we've exceeded a
        // flow of 3
        loop {
            pred.clear();
            q.clear();
            q.push_back(source);

            while let Some(u) = q.pop_front() {
                if pred.contains_key(&sink) {
                    flow += 1;
                    break;
                }

                for &v in &self.graph[&u] {
                    if !pred.contains_key(&v) && v != source && !visited_edges.contains(&(u, v)) {
                        pred.insert(v, u);
                        q.push_back(v)
                    }
                }
            }

            // if flow is > 3, it means the source and sink we've chosen are not correct, we can
            // just return early in this case
            if flow > 3 {
                return None;
            }

            // sink is unreachable, don't search further
            if !pred.contains_key(&sink) {
                break;
            }

            // we know the flow is always 1, so we simplify updating the residual network to just
            // insert visited edges
            let mut v = sink;
            while let Some(&u) = pred.get(&v) {
                visited_edges.insert((u, v));
                v = u;
            }
        }

        // we probably never hit this, but just in case if for whatever reason the min cut is
        // actually < 3?
        if flow != 3 {
            return None;
        }

        // now we just need to do bfs from the source once while avoiding the edges that have
        // already reached capacity (visited_edges). since we've found the max flow, all the min
        // cut edges should be saturated, which means our bfs will only reach 1 of the 2 islands.
        let mut visited_vertices = FxHashSet::default();
        let mut q = VecDeque::default();
        q.push_back(source);
        visited_vertices.insert(source);

        while let Some(u) = q.pop_front() {
            for &v in &self.graph[&u] {
                if !visited_vertices.contains(&v)
                    && !visited_edges.contains(&(u, v))
                    && !visited_edges.contains(&(v, u))
                {
                    q.push_back(v);
                    visited_vertices.insert(v);
                }
            }
        }

        let count = visited_vertices.len();
        Some(count * (self.graph.len() - count))
    }
}

impl FromStr for Snowverload {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph: Graph = FxHashMap::default();

        for line in s.lines() {
            if let Some((left, right)) = line.split_once(": ") {
                let v = u16::from_str_radix(left, 36)?;

                for token in right.split_whitespace() {
                    let u = u16::from_str_radix(token, 36)?;

                    graph
                        .entry(v)
                        .and_modify(|x| {
                            x.push(u);
                        })
                        .or_insert(vec![u]);

                    graph
                        .entry(u)
                        .and_modify(|x| {
                            x.push(v);
                        })
                        .or_insert(vec![v]);
                }
            }
        }

        let vertices = graph.keys().copied().collect();
        Ok(Self { graph, vertices })
    }
}

impl Problem for Snowverload {
    const DAY: usize = 25;
    const TITLE: &'static str = "snowverload";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.min_cut()
            .ok_or_else(|| anyhow!("count not find answer"))
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
        let solution = Snowverload::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(554064, 0));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = Snowverload::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(54, 0));
    }
}
