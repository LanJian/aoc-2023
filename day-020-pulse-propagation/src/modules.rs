use anyhow::bail;
use rustc_hash::FxHashMap;
use std::{collections::VecDeque, str::FromStr};

use crate::Signal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Pulse {
    High,
    Low,
}

impl From<bool> for Pulse {
    fn from(value: bool) -> Self {
        match value {
            true => Self::High,
            false => Self::Low,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Module {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

impl Module {
    pub const BUTTON_ID: u16 = 0;
    pub const BROADCASTER_ID: u16 = 1;
    pub const RX_ID: u16 = 1005;

    pub fn process(&mut self, signal: &Signal, q: &mut VecDeque<Signal>) {
        match self {
            Self::Broadcaster(x) => x.process(q),
            Self::FlipFlop(x) => x.process(signal.pulse, q),
            Self::Conjunction(x) => x.process(signal.source, signal.pulse, q),
        }
    }

    pub fn outputs(&self) -> &Vec<u16> {
        match self {
            Self::Broadcaster(x) => &x.outputs,
            Self::FlipFlop(x) => &x.outputs,
            Self::Conjunction(x) => &x.outputs,
        }
    }

    pub fn reset(&mut self) {
        match self {
            Self::Broadcaster(_) => (),
            Self::FlipFlop(x) => x.reset(),
            Self::Conjunction(x) => x.reset(),
        }
    }
}

impl FromStr for Module {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((label, labels)) = s.split_once(" -> ") {
            if label == "broadcaster" {
                let outputs = labels
                    .split(", ")
                    .map(|x| u16::from_str_radix(x, 36))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Broadcaster(Broadcaster { outputs }))
            } else if let Some(stripped) = label.strip_prefix('%') {
                let id = u16::from_str_radix(stripped, 36)?;
                let outputs = labels
                    .split(", ")
                    .map(|x| u16::from_str_radix(x, 36))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::FlipFlop(FlipFlop {
                    id,
                    outputs,
                    power: false,
                }))
            } else if let Some(stripped) = label.strip_prefix('&') {
                let id = u16::from_str_radix(stripped, 36)?;
                let outputs = labels
                    .split(", ")
                    .map(|x| u16::from_str_radix(x, 36))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Conjunction(Conjunction {
                    id,
                    outputs,
                    cache: FxHashMap::default(),
                }))
            } else {
                bail!("invalid module")
            }
        } else {
            bail!("invalid line")
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Broadcaster {
    outputs: Vec<u16>,
}

impl Broadcaster {
    pub fn process(&self, q: &mut VecDeque<Signal>) {
        for id in &self.outputs {
            q.push_back(Signal::new(Module::BROADCASTER_ID, *id, Pulse::Low));
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FlipFlop {
    pub(crate) id: u16,
    pub power: bool,
    outputs: Vec<u16>,
}
impl FlipFlop {
    pub fn process(&mut self, pulse: Pulse, q: &mut VecDeque<Signal>) {
        if pulse == Pulse::High {
            return;
        }

        self.power = !self.power;
        let out = Pulse::from(self.power);
        for id in &self.outputs {
            q.push_back(Signal::new(self.id, *id, out));
        }
    }

    fn reset(&mut self) {
        self.power = false;
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Conjunction {
    pub(crate) id: u16,
    pub cache: FxHashMap<u16, Pulse>,
    outputs: Vec<u16>,
}
impl Conjunction {
    pub fn process(&mut self, source: u16, pulse: Pulse, q: &mut VecDeque<Signal>) {
        self.cache.entry(source).and_modify(|x| *x = pulse);

        let out = if self.cache.values().all(|x| *x == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        };

        for id in &self.outputs {
            q.push_back(Signal::new(self.id, *id, out));
        }
    }

    fn reset(&mut self) {
        for (_, pulse) in self.cache.iter_mut() {
            *pulse = Pulse::Low;
        }
    }
}
