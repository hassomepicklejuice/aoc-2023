use std::{collections::HashMap, io};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = part1(&input);
    println!("part1: {output:?}");
    let output = part2(&input);
    println!("part2: {output:?}");
}

fn parse(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut lines = input.trim().lines();
    let instructions = lines.next().unwrap().trim();
    lines.next();
    let map = lines
        .flat_map(|line| {
            let (position, connections) = line.split_once('=')?;
            let (left, right) = connections
                .trim()
                .trim_matches(&['(', ')'] as &[_])
                .split_once(',')?;
            Some((position.trim(), (left.trim(), right.trim())))
        })
        .collect();
    (instructions, map)
}

fn part1(input: &str) -> usize {
    let (instructions, map) = parse(input);
    let mut curr = "AAA";
    let goal = "ZZZ";
    for (idx, inst) in instructions.chars().cycle().enumerate() {
        if curr == goal {
            return idx;
        }
        let (left, right) = map[curr];
        match inst {
            'L' => {
                curr = left;
            }
            'R' => {
                curr = right;
            }
            _ => unreachable!(),
        }
    }
    unreachable!()
}

fn part2(input: &str) -> usize {
    let (instructions, map) = parse(input);
    let startings: Vec<_> = map
        .keys()
        .copied()
        .filter(|pos| pos.ends_with('A'))
        .collect();
    startings
        .into_iter()
        .map(|mut curr| {
            dbg!(&curr);
            for (idx, inst) in instructions.chars().cycle().enumerate() {
                if curr.ends_with('Z') {
                    return idx;
                }
                let (left, right) = map[curr];
                curr = match inst {
                    'L' => left,
                    'R' => right,
                    _ => unreachable!(),
                }
            }
            unreachable!()
        })
        .fold(1, lcm)
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[test]
fn test1() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    let output = part1(input);
    assert_eq!(output, 2);
}
