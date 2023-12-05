use std::{collections::HashMap, io, str::FromStr};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = part1(&input);
    println!("part1: {output}");
    let output = part2(&input);
    println!("part2: {output}");
}

#[derive(Debug)]
struct Range {
    dest_start: usize,
    source_start: usize,
    range_len: usize,
}

impl Range {
    fn map(&self, source: usize) -> Option<usize> {
        let diff = source.wrapping_sub(self.source_start);
        if diff < self.range_len {
            Some(self.dest_start + diff)
        } else {
            None
        }
    }
    fn from_line(input: &str) -> Self {
        let mut iter = input.split_whitespace();
        Self {
            dest_start: iter.next().unwrap().parse().unwrap(),
            source_start: iter.next().unwrap().parse().unwrap(),
            range_len: iter.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn get_mapped(&self, source: usize) -> usize {
        self.ranges
            .iter()
            .flat_map(|r| r.map(source))
            .next()
            .unwrap_or(source)
    }
    fn from_block(input: &str) -> (&str, &str, Self) {
        let mut lines = input.lines();
        let (kind, _) = lines.next().unwrap().split_once(' ').unwrap();
        let (source, dest) = kind.split_once("-to-").unwrap();
        let ranges = lines.map(Range::from_line).collect();
        (source, dest, Map { ranges })
    }
}

fn part1(input: &str) -> usize {
    let mut blocks = input.split("\n\n");
    let seeds = blocks.next().unwrap();

    let seeds = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(FromStr::from_str);

    let maps: HashMap<&str, (&str, Map)> = blocks
        .map(Map::from_block)
        .map(|(src, dst, map)| (src, (dst, map)))
        .collect();

    seeds
        .map(|mut s| {
            let mut curr_cat = "seed";
            while let Some((dest, map)) = maps.get(curr_cat) {
                s = map.get_mapped(s);
                curr_cat = dest;
            }
            s
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let mut blocks = input.split("\n\n");
    let seeds = blocks.next().unwrap();

    let seeds = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(FromStr::from_str)
        .collect::<Vec<usize>>();

    let seeds = seeds
        .chunks(2)
        .inspect(|r| eprintln!("seed range: {r:?}"))
        .flat_map(|r| r[0]..(r[0] + r[1]));

    // Very slow -> create a single map from seed to location
    let maps: Vec<Map> = blocks.map(Map::from_block).map(|(_, _, map)| map).collect();

    seeds
        .map(|mut s| {
            for map in maps.iter() {
                s = map.get_mapped(s);
            }
            s
        })
        .min()
        .unwrap()
}

#[test]
fn test1() {
    let output = part1(EXAMPLE);
    assert_eq!(output, 35);
}

#[test]
fn test2() {
    let output = part2(EXAMPLE);
    assert_eq!(output, 46);
}

const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
