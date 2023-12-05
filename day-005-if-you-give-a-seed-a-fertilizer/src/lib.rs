use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
struct Mapping {
    source: usize,
    destination: usize,
    length: usize,
}

impl Mapping {
    /// Returns the mapped destination value
    ///
    /// If `check_range` is true:
    ///
    /// Returns the mapped destination value if the source value falls within the mapping range.
    /// Returns None otherwise
    ///
    /// If `check_range` is false:
    ///
    /// Returns a mapped destination value regardless if the source value falls with the mapping
    /// range
    fn map(&self, source_value: usize, check_range: bool) -> Option<usize> {
        if !check_range || (source_value >= self.source && source_value < self.source + self.length)
        {
            Some(source_value - self.source + self.destination)
        } else {
            None
        }
    }
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s
            .splitn(3, ' ')
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;
        Ok(Mapping {
            destination: tokens[0],
            source: tokens[1],
            length: tokens[2],
        })
    }
}

#[derive(Debug, Clone)]
pub struct IfYouGiveASeedAFertilizer {
    seeds: Vec<usize>,
    mappings: [Vec<Mapping>; 7],
}

impl IfYouGiveASeedAFertilizer {
    fn seed_to_location(&self, seed: usize) -> usize {
        let mut value = seed;
        for mapping_group in &self.mappings {
            value = mapping_group
                .iter()
                .find_map(|x| x.map(value, true))
                .unwrap_or(value);
        }

        value
    }

    fn min_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|x| self.seed_to_location(*x))
            .min()
            .unwrap_or_default()
    }

    /// Give a list of seed ranges, and a list of mappings, return a list of mapped seed ranges
    ///
    /// Both inputs and the outputs are disjoint and sorted
    fn map_seeds(seed_ranges: &[(usize, usize)], mappings: &[Mapping]) -> Vec<(usize, usize)> {
        let mut ret = Vec::default();
        let mut j = 0;

        for seed_range in seed_ranges {
            let &(mut s, e) = seed_range;

            while s < e {
                while j < mappings.len() && mappings[j].source + mappings[j].length <= s {
                    j += 1;
                }

                if j >= mappings.len() {
                    // we saw all the mappings, so this range won't get mapped
                    ret.push((s, e));
                    break;
                }

                let mapping = &mappings[j];
                let (ms, me) = (mapping.source, mapping.source + mapping.length);

                if ms >= e {
                    // segment is below the mapping range
                    ret.push((s, e));
                    break;
                }

                if ms > s {
                    ret.push((s, ms));
                }

                // we can unwrap because its guaranteed to be in mapping range
                ret.push((
                    mapping.map(s.max(ms), false).unwrap(),
                    mapping.map(e.min(me), false).unwrap(),
                ));

                s = me;
            }
        }

        ret
    }

    fn min_location_with_seed_ranges(&mut self) -> usize {
        let mut seed_ranges: Vec<_> = self.seeds.chunks(2).map(|x| (x[0], x[0] + x[1])).collect();
        seed_ranges.sort();

        for mapping_group in &mut self.mappings {
            mapping_group.sort();
            seed_ranges = Self::map_seeds(&seed_ranges, mapping_group);
            seed_ranges.sort();
        }

        seed_ranges
            .iter()
            .min()
            .map(|(s, _)| *s)
            .unwrap_or_default()
    }
}

impl FromStr for IfYouGiveASeedAFertilizer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let seeds = lines
            .next()
            .and_then(|l| {
                l.split(' ')
                    .skip(1)
                    .map(|x| x.parse::<usize>())
                    .collect::<Result<Vec<usize>, _>>()
                    .ok()
            })
            .ok_or_else(|| anyhow!("could not parse seeds"))?;

        let mut mappings: [Vec<Mapping>; 7] = Default::default();

        lines.next();

        for mapping_group in &mut mappings {
            lines.next();
            let mut l = lines.next();

            while l.is_some_and(|x| !x.is_empty()) {
                // safe to unwrap here
                mapping_group.push(Mapping::from_str(l.unwrap())?);
                l = lines.next();
            }
        }

        Ok(IfYouGiveASeedAFertilizer { seeds, mappings })
    }
}

impl Problem for IfYouGiveASeedAFertilizer {
    const DAY: usize = 5;
    const TITLE: &'static str = "if you give a seed a fertilizer";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.min_location())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.min_location_with_seed_ranges())
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
        let solution = IfYouGiveASeedAFertilizer::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(3374647, 6082852));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = IfYouGiveASeedAFertilizer::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(35, 46));
    }
}
