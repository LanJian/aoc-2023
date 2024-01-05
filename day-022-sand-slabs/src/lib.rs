use std::{collections::VecDeque, str::FromStr};

use anyhow::bail;
use aoc_common::algebra::{Point2, Point3};
use aoc_plumbing::Problem;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
struct Slab {
    a: Point3<i64>,
    b: Point3<i64>,
}

impl Slab {
    fn top(&self) -> i64 {
        self.a.z.max(self.b.z)
    }

    fn bottom(&self) -> i64 {
        self.a.z.min(self.b.z)
    }

    fn drop(&mut self, z: i64) {
        let drop_by = self.bottom() - z - 1;
        self.a.z -= drop_by;
        self.b.z -= drop_by;
    }

    fn points(&self) -> Vec<Point3<i64>> {
        let (min_x, max_x) = (self.a.x.min(self.b.x), self.a.x.max(self.b.x));
        let (min_y, max_y) = (self.a.y.min(self.b.y), self.a.y.max(self.b.y));
        let (min_z, max_z) = (self.a.z.min(self.b.z), self.a.z.max(self.b.z));

        let mut ret = Vec::default();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    ret.push(Point3::new(x, y, z));
                }
            }
        }

        ret
    }
}

impl FromStr for Slab {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((l, r)) = s.split_once('~') {
            let a_coords = l
                .split(',')
                .map(|x| x.parse())
                .collect::<Result<Vec<_>, _>>()?;
            let a = Point3::new(a_coords[0], a_coords[1], a_coords[2]);
            let b_coords = r
                .split(',')
                .map(|x| x.parse())
                .collect::<Result<Vec<_>, _>>()?;
            let b = Point3::new(b_coords[0], b_coords[1], b_coords[2]);
            Ok(Self { a, b })
        } else {
            bail!("invalid slab")
        }
    }
}

#[derive(Debug, Clone)]
pub struct SandSlabs {
    slabs: Vec<Slab>,
    supports: FxHashMap<usize, FxHashSet<usize>>,
    supported_by: FxHashMap<usize, FxHashSet<usize>>,
    cant_remove: FxHashSet<usize>,
}

impl SandSlabs {
    fn disintegratable(&mut self) -> usize {
        let mut heightmap: FxHashMap<Point2<i64>, (i64, usize)> = FxHashMap::default();

        for (i, slab) in self.slabs.iter_mut().enumerate() {
            self.supports.insert(i, FxHashSet::default());
            self.supported_by.insert(i, FxHashSet::default());

            let points = slab.points();
            let z = points
                .iter()
                .map(|&p| {
                    heightmap
                        .get(&Point2::from(p))
                        .map(|&(h, _)| h)
                        .unwrap_or_default()
                })
                .max()
                .unwrap_or_default();

            for p in &points {
                if let Some(&(h, id)) = heightmap.get(&Point2::from(*p)) {
                    if h == z {
                        self.supports.entry(id).and_modify(|x| {
                            x.insert(i);
                        });
                        self.supported_by.entry(i).and_modify(|x| {
                            x.insert(id);
                        });
                    }
                }
            }

            slab.drop(z);

            for p in &slab.points() {
                heightmap.insert(Point2::from(*p), (slab.top(), i));
            }
        }

        for v in self.supported_by.values() {
            if v.len() == 1 {
                self.cant_remove.extend(v);
            }
        }

        self.slabs.len() - self.cant_remove.len()
    }

    fn remove(&self) -> usize {
        self.cant_remove.iter().map(|x| self.remove_one(*x)).sum()
    }

    fn remove_one(&self, to_remove: usize) -> usize {
        let mut supported_by = self.supported_by.clone();
        let mut q = VecDeque::default();
        let mut ret = 0;

        q.push_back(to_remove);

        while let Some(n) = q.pop_front() {
            // for each node m with an edge e from n to m
            for m in &self.supports[&n] {
                // remove edge e from the graph
                supported_by.entry(*m).and_modify(|x| {
                    x.remove(&n);
                });

                // if m has no other incoming edges then insert m into q
                if supported_by[m].is_empty() {
                    q.push_back(*m);

                    // also track this brick as fallen
                    ret += 1;
                }
            }
        }

        ret
    }
}

impl FromStr for SandSlabs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut slabs = s
            .lines()
            .map(Slab::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        slabs.sort_by_key(|x| x.bottom());

        Ok(Self {
            slabs,
            supports: FxHashMap::default(),
            supported_by: FxHashMap::default(),
            cant_remove: FxHashSet::default(),
        })
    }
}

impl Problem for SandSlabs {
    const DAY: usize = 22;
    const TITLE: &'static str = "sand slabs";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.disintegratable())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.remove())
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
        let solution = SandSlabs::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(426, 61920));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = SandSlabs::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(5, 7));
    }
}
