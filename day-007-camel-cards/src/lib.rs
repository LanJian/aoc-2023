use std::str::FromStr;

use anyhow::bail;
use aoc_plumbing::Problem;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Copy, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Default for Card {
    fn default() -> Self {
        Self::Two
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::Joker, // use O to denote joker
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum HandKind {
    Unknown,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    fn from_cards(cards: &[Card; 5]) -> Self {
        let mut freq: FxHashMap<Card, usize> = FxHashMap::default();
        let mut freq_jokers = 0;
        let mut max_card = &cards[0];
        let mut max_freq = 0;
        for card in cards {
            match card {
                Card::Joker => freq_jokers += 1,
                _ => {
                    let freq = freq.entry(*card).and_modify(|x| *x += 1).or_insert(1);
                    if *freq > max_freq {
                        max_freq = *freq;
                        max_card = card;
                    }
                }
            }
        }

        // distribute jokers
        if max_freq > 0 {
            // have jokers mimic the highest freq card
            if let Some(x) = freq.get_mut(max_card) {
                *x += freq_jokers;
            }
        } else {
            // its a hand of five jokers, so five of a kind
            return Self::FiveOfAKind;
        }

        let mut num_pairs = 0;
        let mut num_triples = 0;
        for val in freq.values() {
            match val {
                5 => return Self::FiveOfAKind,
                4 => return Self::FourOfAKind,
                3 => num_triples += 1,
                2 => num_pairs += 1,
                _ => (),
            }
        }

        if num_triples == 1 && num_pairs == 1 {
            Self::FullHouse
        } else if num_triples == 1 && num_pairs == 0 {
            Self::ThreeOfAKind
        } else if num_pairs == 2 {
            Self::TwoPair
        } else if num_pairs == 1 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Hand {
    kind: HandKind,
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn determine_kind(&mut self) {
        self.kind = HandKind::from_cards(&self.cards);
    }

    fn jacks_to_joker(&mut self) {
        for x in self.cards.iter_mut() {
            if *x == Card::Jack {
                *x = Card::Joker;
            }
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s.split_once(' ') {
            let mut cards = [Card::default(); 5];

            for (i, c) in left.chars().take(5).enumerate() {
                let card = c.into();
                cards[i] = card;
            }
            Ok(Hand {
                kind: HandKind::Unknown,
                cards,
                bid: right.parse()?,
            })
        } else {
            bail!("could not parse hand and bid")
        }
    }
}

#[derive(Debug, Clone)]
pub struct CamelCards {
    hands: Vec<Hand>,
}

impl CamelCards {
    fn winnings(&mut self) -> usize {
        let mut ret = 0;

        self.hands.iter_mut().for_each(|x| x.determine_kind());
        self.hands.sort();

        for (i, x) in self.hands.iter().enumerate() {
            ret += (i + 1) * x.bid;
        }

        ret
    }

    fn winnings_with_jokers(&mut self) -> usize {
        let mut ret = 0;

        self.hands.iter_mut().for_each(|x| {
            x.jacks_to_joker();
            x.determine_kind();
        });
        self.hands.sort();

        for (i, x) in self.hands.iter().enumerate() {
            ret += (i + 1) * x.bid;
        }

        ret
    }
}

impl FromStr for CamelCards {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s
            .lines()
            .map(Hand::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { hands })
    }
}

impl Problem for CamelCards {
    const DAY: usize = 7;
    const TITLE: &'static str = "camel cards";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.winnings())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.winnings_with_jokers())
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
        let solution = CamelCards::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(249726565, 251135960));
    }

    #[test]
    fn order_test() {
        assert!(Card::Ace > Card::King);
        assert!(Card::Ace > Card::Two);
        assert!(Card::King > Card::Two);
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = CamelCards::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(6440, 5905));
    }
}
