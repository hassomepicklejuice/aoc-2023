use std::{collections::HashMap, io, str::FromStr};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = part1(&input);
    println!("part1: {output}");
    // let output = part2(&input);
    // println!("part2: {output}");
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct RangeMap {
    source_start: usize,
    range_len: usize,
    dest_start: usize,
}

impl RangeMap {
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
    ranges: Vec<RangeMap>,
}

impl Map {
    fn sort_unstable(&mut self) {
        self.ranges.sort_unstable();
    }
    fn map_val(&self, source: usize) -> usize {
        self.ranges
            .iter()
            .flat_map(|r| r.map(source))
            .next()
            .unwrap_or(source)
    }
    /// Assumes sorted ranges.
    fn map_ranges(&self, other: &Self) -> Self {
        let mut ranges = Vec::with_capacity(other.ranges.len());
        for range in other.ranges.iter() {
            dbg!(&range);
            let res = self.ranges.binary_search(&range).unwrap_or_else(|n| n - 1);
            let lower = &self.ranges[res];
            dbg!(lower);
        }
        Self { ranges }
    }
    fn from_block(input: &str) -> (&str, &str, Self) {
        let mut lines = input.lines();
        let (kind, _) = lines.next().unwrap().split_once(' ').unwrap();
        let (source, dest) = kind.split_once("-to-").unwrap();
        let mut map = Self {
            ranges: lines.map(RangeMap::from_line).collect(),
        };
        map.sort_unstable();
        (source, dest, map)
    }
    fn min(&self) -> usize {
        todo!()
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
                s = map.map_val(s);
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

    let seeds: Vec<RangeMap> = seeds
        .chunks(2)
        .map(|r| RangeMap {
            dest_start: r[0],
            source_start: r[0],
            range_len: r[1],
        })
        .collect();
    let mut seeds: Map = Map { ranges: seeds };

    // Very slow -> create a single map from seed to location
    // or reverse mapping?
    // or boundary mapping/analysis
    let maps: Vec<Map> = blocks.map(Map::from_block).map(|(_, _, map)| map).collect();

    for map in maps {
        seeds = map.map_ranges(&seeds);
    }

    seeds.min()
}

fn _part2(input: &str) -> usize {
    let mut blocks = input.split("\n\n");
    let seeds = blocks.next().unwrap();

    let seeds = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(FromStr::from_str)
        .collect::<Vec<usize>>();

    let seeds = seeds.chunks(2).flat_map(|r| r[0]..(r[0] + r[1]));

    // Very slow -> create a single map from seed to location
    // or reverse mapping?
    // or boundary mapping/analysis
    let maps: Vec<Map> = blocks.map(Map::from_block).map(|(_, _, map)| map).collect();

    seeds
        .map(|mut s| {
            for map in maps.iter() {
                s = map.map_val(s);
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
