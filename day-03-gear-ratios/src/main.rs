use std::{collections::HashMap, io};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output1 = part1(&input);
    println!("part1:\t{output1}");
    let output2 = part2(&input);
    println!("part2:\t{output2}");
}

/// (value, row, start, end)
type Number = (usize, usize, usize, usize);

/// (value, row, col)
type Symbol = (char, usize, usize);

fn part1(input: &str) -> usize {
    let (nums, syms) = parse_input(input);
    let syms: HashMap<_, _> = syms
        .into_iter()
        .map(|(val, row, col)| ((row, col), val))
        .collect();
    let mut sum = 0;
    'outer: for (val, row, start, end) in nums {
        for i in row.saturating_sub(1)..=(row + 1) {
            for j in start.saturating_sub(1)..(end + 1) {
                if syms.get(&(i, j)).is_some() {
                    sum += val;
                    continue 'outer;
                }
            }
        }
    }
    sum
}

fn part2(input: &str) -> usize {
    let (nums, syms) = parse_input(input);
    let mut sum = 0;

    for (sym, row, col) in syms.into_iter().filter(|(s, _, _)| *s == '*') {
        let mut num_count = 0;
        let mut gear_ratio = 1;
        for (num, r, s, e) in nums.iter() {
            if (row.max(*r) - row.min(*r)) <= 1 && col >= s.saturating_sub(1) && col <= *e {
                num_count += 1;
                gear_ratio *= num;
            }
        }
        if num_count == 2 {
            sum += gear_ratio;
        }
    }
    sum
}

fn parse_input(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut nums = vec![];
    let mut syms = vec![];
    for (row, line) in input.lines().enumerate() {
        let tokens = line
            .char_indices()
            .fold(vec![(0, String::new())], |mut acc, (idx, curr)| {
                if curr.is_ascii_digit() {
                    acc.last_mut().map(|(i, s)| {
                        if s.is_empty() {
                            *i = idx;
                        }
                        s.push(curr);
                    });
                } else if curr != '.' {
                    acc.push((idx, curr.to_string()));
                    acc.push((idx, String::new()));
                } else if acc.last().is_some_and(|(_, s)| !s.is_empty()) {
                    acc.push((idx, String::new()));
                }
                acc
            });
        for (idx, s) in tokens.into_iter().filter(|(_, s)| !s.is_empty()) {
            if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                nums.push((s.parse().unwrap(), row, idx, idx + s.len()));
            } else {
                syms.push((s.chars().next().unwrap(), row, idx));
            }
        }
    }
    (nums, syms)
}

#[test]
fn test1() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let output = part1(input);
    assert_eq!(output, 4361);
}

#[test]
fn test2() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let output = part2(input);
    assert_eq!(output, 467835);
}
