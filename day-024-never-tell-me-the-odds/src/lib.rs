use std::str::FromStr;

use anyhow::{anyhow, Result};
use aoc_common::{
    algebra::{Point3, Ray, Vector3},
    geometry::IntersectRay,
};
use aoc_plumbing::Problem;
use nalgebra::{Matrix6, Vector6};

#[derive(Debug, Clone)]
pub struct NeverTellMeTheOdds {
    rays: Vec<Ray<i64>>,
}

impl NeverTellMeTheOdds {
    fn determine_rock(&self) -> Result<i64> {
        let (p1, v1) = (self.rays[0].origin, self.rays[0].dir);
        let (p2, v2) = (self.rays[1].origin, self.rays[1].dir);
        let (p3, v3) = (self.rays[2].origin, self.rays[2].dir);

        let a = Matrix6::new(
            0.0,
            (v2.z - v1.z) as f64,
            (v1.y - v2.y) as f64,
            0.0,
            (p1.z - p2.z) as f64,
            (p2.y - p1.y) as f64,
            0.0,
            (v3.z - v1.z) as f64,
            (v1.y - v3.y) as f64,
            0.0,
            (p1.z - p3.z) as f64,
            (p3.y - p1.y) as f64,
            (v1.z - v2.z) as f64,
            0.0,
            (v2.x - v1.x) as f64,
            (p2.z - p1.z) as f64,
            0.0,
            (p1.x - p2.x) as f64,
            (v1.z - v3.z) as f64,
            0.0,
            (v3.x - v1.x) as f64,
            (p3.z - p1.z) as f64,
            0.0,
            (p1.x - p3.x) as f64,
            (v2.y - v1.y) as f64,
            (v1.x - v2.x) as f64,
            0.0,
            (p1.y - p2.y) as f64,
            (p2.x - p1.x) as f64,
            0.0,
            (v3.y - v1.y) as f64,
            (v1.x - v3.x) as f64,
            0.0,
            (p1.y - p3.y) as f64,
            (p3.x - p1.x) as f64,
            0.0,
        );

        let b = Vector6::new(
            (p1.z * v1.y + p2.y * v2.z - p1.y * v1.z - p2.z * v2.y) as f64,
            (p1.z * v1.y + p3.y * v3.z - p1.y * v1.z - p3.z * v3.y) as f64,
            (p1.x * v1.z + p2.z * v2.x - p1.z * v1.x - p2.x * v2.z) as f64,
            (p1.x * v1.z + p3.z * v3.x - p1.z * v1.x - p3.x * v3.z) as f64,
            (p1.y * v1.x + p2.x * v2.y - p1.x * v1.y - p2.y * v2.x) as f64,
            (p1.y * v1.x + p3.x * v3.y - p1.x * v1.y - p3.y * v3.x) as f64,
        );

        let x = a
            .try_inverse()
            .ok_or_else(|| anyhow!("matrix not invertible"))?
            * b;
        Ok(x[0].round() as i64 + x[1].round() as i64 + x[2].round() as i64)
    }

    fn intersections_2d(&self, min: f64, max: f64) -> usize {
        let mut ret = 0;

        for i in 0..self.rays.len() {
            for j in i + 1..self.rays.len() {
                let (o1, d1) = (self.rays[i].origin, self.rays[i].dir);
                let (o2, d2) = (self.rays[j].origin, self.rays[j].dir);
                let a = Ray::new(
                    Point3::new(o1.x as f64, o1.y as f64, 0.0),
                    Vector3::new(d1.x as f64, d1.y as f64, 0.0),
                );
                let b = Ray::new(
                    Point3::new(o2.x as f64, o2.y as f64, 0.0),
                    Vector3::new(d2.x as f64, d2.y as f64, 0.0),
                );

                if let Some(s) = a.intersect(&b) {
                    let p = s.position;
                    if p.x >= min && p.x <= max && p.y >= min && p.y <= max {
                        ret += 1
                    }
                }
            }
        }

        ret
    }
}

impl FromStr for NeverTellMeTheOdds {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rays = Vec::default();
        for line in s.lines() {
            if let Some((left, right)) = line.split_once('@') {
                let p: Vec<i64> = left
                    .trim()
                    .split(',')
                    .map(|x| x.trim().parse())
                    .collect::<Result<Vec<_>, _>>()?;

                let v: Vec<i64> = right
                    .trim()
                    .split(',')
                    .map(|x| x.trim().parse())
                    .collect::<Result<Vec<_>, _>>()?;

                rays.push(Ray::new(
                    Point3::new(p[0], p[1], p[2]),
                    Vector3::new(v[0], v[1], v[2]),
                ));
            }
        }

        Ok(Self { rays })
    }
}

impl Problem for NeverTellMeTheOdds {
    const DAY: usize = 24;
    const TITLE: &'static str = "never tell me the odds";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.intersections_2d(200000000000000.0, 400000000000000.0))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.determine_rock()
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
        let solution = NeverTellMeTheOdds::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(18651, 546494494317645));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let mut instance = NeverTellMeTheOdds::instance(&input).unwrap();
        assert_eq!(instance.intersections_2d(7.0, 27.0), 2);
        assert_eq!(instance.part_two().unwrap(), 47)
    }
}
