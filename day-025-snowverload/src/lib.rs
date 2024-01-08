use std::str::FromStr;

use anyhow::{anyhow, Result};
use aoc_plumbing::Problem;
use rand::{distributions::WeightedIndex, prelude::Distribution, rngs::ThreadRng, thread_rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use rustc_hash::FxHashMap;

type Graph = FxHashMap<u16, FxHashMap<u16, usize>>;

#[derive(Debug, Clone)]
pub struct Snowverload {
    graph: Graph,
}

impl Snowverload {
    fn min_cut(&self) -> Result<usize> {
        let result = (0..5000).into_par_iter().find_map_any(|_| {
            let mut rng = thread_rng();
            let mut g = self.graph.clone();
            let mut vertices: FxHashMap<u16, usize> = g.keys().map(|x| (*x, 1)).collect();

            while vertices.len() > 2 {
                Self::contract(&mut g, &mut vertices, &mut rng);
            }

            // safe to unwrap because we know theres 2 vertices with edges between them
            if *g
                .values()
                .next()
                .expect("could not get vertex")
                .values()
                .next()
                .expect("could not get edge count")
                == 3
            {
                Some(vertices.values().product())
            } else {
                None
            }
        });

        result.ok_or_else(|| anyhow!("could not find answer"))
    }

    fn contract(graph: &mut Graph, vertices: &mut FxHashMap<u16, usize>, rng: &mut ThreadRng) {
        let (u, v) = Self::random_edge(graph, rng);

        // move all edges going to v to go to u
        // safe to unwrap, we guarantee the vertex is in the graph
        let edges = graph
            .remove(&v)
            .expect("could not remove contracted vertex");

        for (&k, &count) in &edges {
            if k == u {
                continue;
            }

            // update v -> k edges to u -> k
            graph.entry(u).and_modify(|x| {
                x.entry(k).and_modify(|y| *y += count).or_insert(count);
            });

            // update k -> v edges to k -> u
            graph.entry(k).and_modify(|x| {
                x.entry(u).and_modify(|y| *y += count).or_insert(count);
                x.remove(&v);
            });
        }

        graph.entry(u).and_modify(|x| {
            x.remove(&v);
        });

        let count = vertices[&v];
        vertices.entry(u).and_modify(|x| *x += count);
        vertices.remove(&v);
    }

    fn random_edge(graph: &Graph, rng: &mut ThreadRng) -> (u16, u16) {
        // we have to pick an edge from a weighted distribution
        let mut edges = Vec::default();
        let mut weights = Vec::default();

        for (&u, neighbours) in graph {
            for (&v, &count) in neighbours {
                edges.push((u, v));
                weights.push(count);
            }
        }

        // safe to unwrap because weights are guaranteed to be well-formed
        let dist = WeightedIndex::new(&weights).expect("coult not build distribution");
        edges[dist.sample(rng)]
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
                            x.insert(u, 1);
                        })
                        .or_insert({
                            let mut x = FxHashMap::default();
                            x.insert(u, 1);
                            x
                        });

                    graph
                        .entry(u)
                        .and_modify(|x| {
                            x.insert(v, 1);
                        })
                        .or_insert({
                            let mut x = FxHashMap::default();
                            x.insert(v, 1);
                            x
                        });
                }
            }
        }

        Ok(Self { graph })
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
