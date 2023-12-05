use std::{cmp::Ordering, collections::HashMap, io, ops, str::FromStr};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = part1(&input);
    println!("part1: {output}");
    let output = part2(&input);
    println!("part2: {output}");
}

type Range = ops::Range<usize>;

#[derive(Debug, Default, PartialEq, Eq)]
struct RangeMap {
    src: Range,
    dest: Range,
}

impl PartialOrd for RangeMap {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RangeMap {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_ranges(&self.src, &other.src)
    }
}

fn cmp_ranges(lhs: &Range, rhs: &Range) -> Ordering {
    match lhs.start.cmp(&rhs.start) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => lhs.end.cmp(&rhs.end),
    }
}

impl RangeMap {
    fn map_val(&self, val: usize) -> Option<usize> {
        match self.src.contains(&val) {
            true => Some(val - self.src.start + self.dest.start),
            false => None,
        }
    }
    fn map_range(&self, range: &Range) -> (u8, [Range; 3]) {
        let mut tag = 0;
        let res = [
            // Range before: kept the same
            match range.start < self.src.start {
                true => {
                    tag |= 0b100;
                    range.start..range.end.min(self.src.start)
                }
                false => Default::default(),
            },
            // Range between: mapped
            match range.start < self.src.end && range.end > self.src.start {
                true => {
                    tag |= 0b010;
                    let start = range.start.max(self.src.start);
                    let end = range.end.min(self.src.end);
                    self.map_val(start).unwrap()..(self.map_val(end - 1).unwrap() + 1)
                }
                false => Default::default(),
            },
            // Range after: kept the same
            match range.end >= self.src.end {
                true => {
                    tag |= 0b001;
                    range.start.max(self.src.end)..range.end
                }
                false => Default::default(),
            },
        ];

        (tag, res)
    }

    fn from_line(input: &str) -> Self {
        let mut iter = input.split_whitespace();

        let dest_start = iter.next().unwrap().parse().unwrap();
        let source_start = iter.next().unwrap().parse().unwrap();
        let range_len: usize = iter.next().unwrap().parse().unwrap();

        Self {
            src: source_start..(source_start + range_len),
            dest: dest_start..(dest_start + range_len),
        }
    }
}

#[derive(Debug)]
struct FullMap {
    ranges: Vec<RangeMap>,
}

impl From<Vec<RangeMap>> for FullMap {
    fn from(mut input: Vec<RangeMap>) -> Self {
        input.sort_unstable();
        Self { ranges: input }
    }
}

impl FullMap {
    fn map_val(&self, val: usize) -> usize {
        self.ranges
            .iter()
            .flat_map(|range| range.map_val(val))
            .next()
            .unwrap_or(val)
    }
    fn map_range(&self, mut range: Range) -> Vec<Range> {
        let mut has_after = false;
        let mut res = vec![];
        for r in self.ranges.iter() {
            let (tag, [before, during, after]) = r.map_range(&range);
            if tag & 0b100 > 0 {
                res.push(before);
            }
            if tag & 0b010 > 0 {
                res.push(during);
            }
            if tag & 0b001 > 0 {
                has_after = true;
                range = after;
            } else {
                has_after = false;
                break;
            }
        }
        if has_after {
            res.push(range);
        }
        res
    }
    fn from_block(input: &str) -> (&str, &str, Self) {
        let mut lines = input.lines();
        let (kind, _) = lines.next().unwrap().split_once(' ').unwrap();
        let (source, dest) = kind.split_once("-to-").unwrap();
        let mut map = Self::from(lines.map(RangeMap::from_line).collect::<Vec<RangeMap>>());
        (source, dest, map)
    }
    fn map_ranges(&self, ranges: &[Range]) -> Vec<Range> {
        let mut res = Vec::with_capacity(ranges.len());
        for range in ranges {
            res.append(&mut self.map_range(range.clone()));
        }
        res.sort_unstable_by(cmp_ranges);
        res
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

    let maps: HashMap<&str, (&str, FullMap)> = blocks
        .map(FullMap::from_block)
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

    let mut seeds: Vec<Range> = seeds.chunks(2).map(|r| r[0]..(r[0] + r[1])).collect();

    // Very slow -> create a single map from seed to location
    // or reverse mapping?
    // or boundary mapping/analysis
    let maps: Vec<FullMap> = blocks
        .map(FullMap::from_block)
        .map(|(_, _, map)| map)
        .collect();

    for map in maps {
        seeds = map.map_ranges(&seeds);
    }

    seeds.first().unwrap().start
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
    let maps: Vec<FullMap> = blocks
        .map(FullMap::from_block)
        .map(|(_, _, map)| map)
        .collect();

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
