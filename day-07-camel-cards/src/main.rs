use std::{cmp::Ordering, collections::BTreeMap, io, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    CJ = 1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CQ,
    CK,
    CA,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '2' => Ok(Self::C2),
            '3' => Ok(Self::C3),
            '4' => Ok(Self::C4),
            '5' => Ok(Self::C5),
            '6' => Ok(Self::C6),
            '7' => Ok(Self::C7),
            '8' => Ok(Self::C8),
            '9' => Ok(Self::C9),
            'T' => Ok(Self::CT),
            'J' => Ok(Self::CJ),
            'Q' => Ok(Self::CQ),
            'K' => Ok(Self::CK),
            'A' => Ok(Self::CA),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand1([Card; 5]);

impl Hand1 {
    fn ty(&self) -> Type {
        let set = self.0.iter().fold(BTreeMap::new(), |mut acc, card| {
            acc.entry(*card).and_modify(|n| *n += 1).or_insert(1);
            acc
        });
        match set.len() {
            1 => Type::FiveOfAKind,
            2 => match set.first_key_value() {
                Some((_, 1)) | Some((_, 4)) => Type::FourOfAKind,
                Some((_, 2)) | Some((_, 3)) => Type::FullHouse,
                _ => unreachable!(),
            },
            3 => match set.iter().max_by_key(|(_, n)| *n) {
                Some((_, 3)) => Type::ThreeOfAKind,
                Some((_, 2)) => Type::TwoPair,
                _ => unreachable!(),
            },
            4 => Type::OnePair,
            5 => Type::HighCard,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Hand1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(());
        }
        let mut cards = [Card::CA; 5];
        for (i, ch) in s.chars().enumerate() {
            cards[i] = ch.try_into()?;
        }
        Ok(Self(cards))
    }
}

impl PartialOrd for Hand1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand1 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.ty().cmp(&other.ty()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (c1, c2) in self.0.iter().zip(other.0.iter()) {
                    let ord = c1.cmp(c2);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
                Ordering::Equal
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand2([Card; 5]);

impl Hand2 {
    fn ty(&self) -> Type {
        let (jokers, set) = self
            .0
            .iter()
            .fold((0, BTreeMap::new()), |(jokers, mut set), card| {
                if *card == Card::CJ {
                    (jokers + 1, set)
                } else {
                    set.entry(*card).and_modify(|n| *n += 1).or_insert(1);
                    (jokers, set)
                }
            });
        match set.iter().max_by_key(|(_, n)| *n) {
            Some((_, c)) if c + jokers == 5 => Type::FiveOfAKind,
            Some((_, c)) if c + jokers == 4 => Type::FourOfAKind,
            Some((_, c)) if c + jokers == 3 => match set.len() {
                2 => Type::FullHouse,
                3 => Type::ThreeOfAKind,
                _ => unreachable!(),
            },
            Some((_, c)) if c + jokers == 2 => match set.len() {
                3 => Type::TwoPair,
                4 => Type::OnePair,
                _ => unreachable!(),
            },
            Some((_, c)) if c + jokers == 1 => Type::HighCard,
            None => Type::FiveOfAKind, // 5 jokers
            _ => unreachable!(),
        }
    }
}

impl FromStr for Hand2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(());
        }
        let mut cards = [Card::CA; 5];
        for (i, ch) in s.chars().enumerate() {
            cards[i] = ch.try_into()?;
        }
        Ok(Self(cards))
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.ty().cmp(&other.ty()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (c1, c2) in self.0.iter().zip(other.0.iter()) {
                    let ord = c1.cmp(c2);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
                Ordering::Equal
            }
        }
    }
}

fn parse_input1(input: &str) -> Vec<(Hand1, usize)> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand = hand.trim().parse().unwrap();
            let bid = bid.trim().parse().unwrap();
            (hand, bid)
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let mut input = parse_input1(input);
    input.sort_unstable_by_key(|(hand, _)| *hand);
    input
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + bid * (i + 1))
}

fn parse_input2(input: &str) -> Vec<(Hand2, usize)> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand = hand.trim().parse().unwrap();
            let bid = bid.trim().parse().unwrap();
            (hand, bid)
        })
        .collect()
}

fn part2(input: &str) -> usize {
    let mut input = parse_input2(input);
    input.sort_unstable_by_key(|(hand, _)| *hand);
    input
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + bid * (i + 1))
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = part1(&input);
    println!("part1:\t{output}");
    let output = part2(&input);
    println!("part2:\t{output}");
}

const EXAMPLE: &'static str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[test]
fn test1() {
    let output = part1(EXAMPLE);
    assert_eq!(output, 6440);
}

#[test]
fn test2() {
    let output = part2(EXAMPLE);
    assert_eq!(output, 5905);
}
