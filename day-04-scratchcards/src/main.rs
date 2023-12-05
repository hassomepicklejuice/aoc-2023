use anyhow::{anyhow, Error, Result};
use std::{io, iter::repeat, str::FromStr};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = part2(&input).unwrap();
    println!("part2:\t{output}");
}

#[derive(Clone, Debug)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    nums: Vec<usize>,
    win_count: Option<usize>,
}

impl Card {
    fn win_count(&mut self) -> usize {
        if let Some(wc) = self.win_count {
            wc
        } else {
            let wc = self
                .winning
                .iter()
                .filter(|n| self.nums.contains(n))
                .count();
            self.win_count = Some(wc);
            wc
        }
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card, nums) = s.split_once(": ").ok_or(anyhow!("Invalid line {s:?}"))?;
        let id: usize = card
            .split_whitespace()
            .nth(1)
            .ok_or(anyhow!("Invalid line {s:?}"))?
            .parse()?;
        let (winning, nums) = nums
            .split_once(" | ")
            .ok_or(anyhow!("Invalid line {s:?}"))?;
        let winning = winning.split_whitespace().flat_map(|n| n.parse()).collect();
        let nums = nums.split_whitespace().flat_map(|n| n.parse()).collect();
        Ok(Self {
            id,
            winning,
            nums,
            win_count: None,
        })
    }
}

fn parse_input(input: &str) -> Result<Vec<Card>> {
    input.lines().map(FromStr::from_str).collect()
}

fn part2(input: &str) -> Result<usize> {
    let input = parse_input(input)?;
    let mut cards: Vec<(Card, usize)> = input.into_iter().zip(repeat(1)).collect();
    for idx in 0..cards.len() {
        let (card, count) = &mut cards[idx];
        let wc = card.win_count();
        let count = *count;
        cards[idx + 1..idx + 1 + wc]
            .iter_mut()
            .for_each(|(_, n)| *n += count);
    }
    Ok(cards.into_iter().map(|(_, count)| count).sum())
}

fn _part2(input: &str) -> usize {
    let mut input = parse_input(input).unwrap();
    let original = input.clone();
    let mut idx = 0;
    while idx < input.len() {
        let card = &input[idx];
        let count = card
            .nums
            .iter()
            .filter(|n| card.winning.contains(n))
            .count();
        input.extend_from_slice(&original[card.id..(card.id + count)]);
        idx += 1;
    }
    idx
}

#[test]
fn test2() {
    let example = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let output = part2(example).expect("30");

    assert_eq!(output, 30);
}
