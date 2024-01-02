use std::{collections::VecDeque, str::FromStr};

use anyhow::Result;
use aoc_plumbing::Problem;
use modules::Pulse;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::modules::Module;

mod modules;

#[derive(Debug, Clone)]
struct Signal {
    source: u16,
    target: u16,
    pulse: Pulse,
}

impl Signal {
    fn new(source: u16, target: u16, pulse: Pulse) -> Self {
        Self {
            source,
            target,
            pulse,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PulsePropagation {
    modules: FxHashMap<u16, Module>,
    penultimate: u16,
}

impl PulsePropagation {
    fn min_presses(&mut self) -> usize {
        let mut round = 0;
        let mut ret = 1;
        let mut seen = FxHashSet::default();

        loop {
            round += 1;
            if let Some(source) = self.min_presses_helper() {
                if seen.contains(&source) {
                    return ret;
                } else {
                    ret *= round;
                    seen.insert(source);
                }
            }
        }
    }

    fn min_presses_helper(&mut self) -> Option<u16> {
        let mut q = VecDeque::default();
        let mut ret = None;

        q.push_back(Signal::new(
            Module::BUTTON_ID,
            Module::BROADCASTER_ID,
            Pulse::Low,
        ));

        while let Some(signal) = q.pop_front() {
            if signal.target == self.penultimate && signal.pulse == Pulse::High {
                ret = Some(signal.source);
            }

            if let Some(module) = self.modules.get_mut(&signal.target) {
                module.process(&signal, &mut q);
            }
        }

        ret
    }

    fn simulate(&mut self, rounds: usize) -> Result<usize> {
        let mut highs = 0;
        let mut lows = 0;

        for _ in 0..rounds {
            let result = self.simulate_one()?;
            highs += result.0;
            lows += result.1;
        }

        Ok(highs * lows)
    }

    fn simulate_one(&mut self) -> Result<(usize, usize)> {
        let mut highs = 0;
        let mut lows = 0;
        let mut q = VecDeque::default();
        q.push_back(Signal::new(
            Module::BUTTON_ID,
            Module::BROADCASTER_ID,
            Pulse::Low,
        ));

        while let Some(signal) = q.pop_front() {
            match signal.pulse {
                Pulse::High => highs += 1,
                Pulse::Low => lows += 1,
            }

            if let Some(module) = self.modules.get_mut(&signal.target) {
                module.process(&signal, &mut q);
            }
        }

        Ok((highs, lows))
    }

    fn reset(&mut self) {
        for (_, module) in self.modules.iter_mut() {
            module.reset();
        }
    }
}

impl FromStr for PulsePropagation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modules = FxHashMap::default();
        let mut edges = Vec::default();
        let mut penultimate = 0;

        for line in s.lines() {
            let module = Module::from_str(line)?;
            let id = match &module {
                Module::Broadcaster(_) => Module::BROADCASTER_ID,
                Module::FlipFlop(x) => x.id,
                Module::Conjunction(x) => x.id,
            };

            for &target in module.outputs() {
                edges.push((id, target));
            }

            if module.outputs().contains(&Module::RX_ID) {
                penultimate = id;
            }

            modules.insert(id, module);
        }

        for (source, target) in edges {
            modules.entry(target).and_modify(|x| {
                if let Module::Conjunction(c) = x {
                    c.cache.insert(source, Pulse::Low);
                }
            });
        }

        Ok(Self {
            modules,
            penultimate,
        })
    }
}

impl Problem for PulsePropagation {
    const DAY: usize = 20;
    const TITLE: &'static str = "pulse propagation";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.simulate(1000)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.reset();
        Ok(self.min_presses())
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
        let solution = PulsePropagation::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(836127690, 240914003753369));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let mut instance = PulsePropagation::instance(&input).unwrap();
        assert_eq!(instance.part_one().unwrap(), 32000000);
    }

    #[test]
    fn example_two() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> out";
        let mut instance = PulsePropagation::instance(input).unwrap();
        assert_eq!(instance.part_one().unwrap(), 11687500);
    }
}
