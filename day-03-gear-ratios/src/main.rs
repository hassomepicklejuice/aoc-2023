use std::{collections::HashMap, io};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output1 = part1(&input);
    println!("part1:\t{output1}");
    let output2 = part2(&input);
    println!("part2:\t{output2}");
}

struct _Number {
    value: usize,
    row: usize,
    start: usize,
    end: usize,
}

struct _Symbol {
    value: char,
    row: usize,
    col: usize,
}

fn part1(input: &str) -> usize {
    let (nums, syms) = parse_input(input);
    let syms: HashMap<_, _> = syms
        .into_iter()
        .map(|_Symbol { value, row, col }| ((row, col), value))
        .collect();
    let mut sum = 0;
    'outer: for num in nums {
        for i in num.row.saturating_sub(1)..=(num.row + 1) {
            for j in num.start.saturating_sub(1)..(num.end + 1) {
                if syms.get(&(i, j)).is_some() {
                    sum += num.value;
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

    for sym in syms.into_iter().filter(|s| s.value == '*') {
        let mut num_count = 0;
        let mut gear_ratio = 1;
        for num in nums.iter() {
            if (sym.row.max(num.row) - sym.row.min(num.row)) <= 1
                && sym.col >= num.start.saturating_sub(1)
                && sym.col <= num.end
            {
                num_count += 1;
                gear_ratio *= num.value;
            }
        }
        if num_count == 2 {
            sum += gear_ratio;
        }
    }
    sum
}

fn parse_input(input: &str) -> (Vec<_Number>, Vec<_Symbol>) {
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
        for (col, s) in tokens.into_iter().filter(|(_, s)| !s.is_empty()) {
            if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                nums.push(_Number {
                    value: s.parse().unwrap(),
                    row,
                    start: col,
                    end: col + s.len(),
                });
            } else {
                syms.push(_Symbol {
                    value: s.chars().next().unwrap(),
                    row,
                    col,
                });
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
