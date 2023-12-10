use std::{cmp, collections::HashMap, io, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
enum Card {
    Joker,
    Numeric2,
    Numeric3,
    Numeric4,
    Numeric5,
    Numeric6,
    Numeric7,
    Numeric8,
    Numeric9,
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '9' => Ok(Card::Numeric9),
            '8' => Ok(Card::Numeric8),
            '7' => Ok(Card::Numeric7),
            '6' => Ok(Card::Numeric6),
            '5' => Ok(Card::Numeric5),
            '4' => Ok(Card::Numeric4),
            '3' => Ok(Card::Numeric3),
            '2' => Ok(Card::Numeric2),
            _ => Err(format!("invalid card: {value}")),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
enum HandResult {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Hand {
    cards: [Card; 5],
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(format!("invalid hand size: {}", s.len()));
        }

        let mut chars = s.chars();

        Ok(Self {
            cards: [
                Card::try_from(chars.next().unwrap())?,
                Card::try_from(chars.next().unwrap())?,
                Card::try_from(chars.next().unwrap())?,
                Card::try_from(chars.next().unwrap())?,
                Card::try_from(chars.next().unwrap())?,
            ],
        })
    }
}

impl Hand {
    fn jokerize(&mut self) {
        for card in &mut self.cards {
            if *card == Card::J {
                *card = Card::Joker;
            }
        }
    }

    fn result(&self) -> HandResult {
        let mut counts: HashMap<&Card, u64> = HashMap::new();
        for card in &self.cards {
            *counts.entry(card).or_default() += 1;
        }

        match counts.len() {
            1 => HandResult::FiveOfAKind,
            2 => {
                if counts.values().any(|count| *count == 4) {
                    if counts.contains_key(&Card::Joker) {
                        HandResult::FiveOfAKind
                    } else {
                        HandResult::FourOfAKind
                    }
                } else {
                    match counts.get(&Card::Joker) {
                        Some(2 | 3) => HandResult::FiveOfAKind,
                        Some(1) => HandResult::FourOfAKind,
                        _ => HandResult::FullHouse,
                    }
                }
            }
            3 => {
                if counts.values().any(|count| *count == 3) {
                    if counts.contains_key(&Card::Joker) {
                        HandResult::FourOfAKind
                    } else {
                        HandResult::ThreeOfAKind
                    }
                } else {
                    match counts.get(&Card::Joker) {
                        Some(2) => HandResult::FourOfAKind,
                        Some(1) => HandResult::FullHouse,
                        _ => HandResult::TwoPair,
                    }
                }
            }
            4 => {
                if counts.contains_key(&Card::Joker) {
                    HandResult::ThreeOfAKind
                } else {
                    HandResult::OnePair
                }
            }
            5 => {
                if counts.contains_key(&Card::Joker) {
                    HandResult::OnePair
                } else {
                    HandResult::HighCard
                }
            }
            _ => panic!("invalid hand: {:?}", self.cards),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.result().cmp(&other.result()) {
            cmp::Ordering::Equal => {
                for i in 0..5 {
                    if self.cards[i] < other.cards[i] {
                        return cmp::Ordering::Less;
                    }
                    if self.cards[i] > other.cards[i] {
                        return cmp::Ordering::Greater;
                    }
                }

                cmp::Ordering::Equal
            }
            o => o,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut hands_and_bets: Vec<(Hand, u64)> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut chunks = line.split_ascii_whitespace();
            (
                Hand::from_str(chunks.next().unwrap()).unwrap(),
                chunks.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    hands_and_bets.sort_by(|a, b| a.0.cmp(&b.0));
    let part1 = hands_and_bets
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand_and_bet)| {
            acc + hand_and_bet.1 * (i as u64 + 1)
        });

    println!("Part 1: {part1}");

    for hand_and_bet in &mut hands_and_bets {
        hand_and_bet.0.jokerize();
    }

    hands_and_bets.sort_by(|a, b| a.0.cmp(&b.0));
    let part2 = hands_and_bets
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand_and_bet)| {
            acc + hand_and_bet.1 * (i as u64 + 1)
        });

    println!("Part 2: {part2}");
}
