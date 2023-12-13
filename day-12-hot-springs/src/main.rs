use std::{fmt::Display, io, str::FromStr};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = part1(&input);
    println!("part1:\t{output}");
    let output = part2(&input);
    println!("part2:\t{output}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Off,
    Unknown,
    On,
}
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Off => '#',
                State::Unknown => '?',
                State::On => '.',
            }
        )
    }
}
impl TryFrom<char> for State {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Off),
            '?' => Ok(Self::Unknown),
            '.' => Ok(Self::On),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Row(Vec<State>);
impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.0.iter() {
            write!(f, "{s}")?;
        }
        Ok(())
    }
}
impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars().map(State::try_from).collect::<Result<_, _>>()?,
        ))
    }
}

fn part1(input: &str) -> usize {
    let rows: Vec<_> = input
        .lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(row, nums)| {
            (
                row.parse::<Row>().unwrap().0,
                nums.split(',')
                    .flat_map(str::parse::<usize>)
                    .collect::<Vec<usize>>(),
            )
        })
        .collect();

    rows.into_iter().map(|(row, nums)| count(&row, &nums)).sum()
}

fn part2(input: &str) -> usize {
    let rows: Vec<_> = input
        .lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(row, nums)| {
            let mut row = row.parse::<Row>().unwrap().0;
            row.push(State::Unknown);
            row = row.repeat(5);
            row.pop();
            let mut nums: Vec<_> = nums.split(',').flat_map(usize::from_str).collect();
            nums = nums.repeat(5);
            (row, nums)
        })
        .collect();

    rows.into_iter().map(|(row, nums)| count(&row, &nums)).sum()
}

/// Whether or not the given block-length `n` fits at the beginning of the row:
/// ```txt
/// +-- n -+ ++
/// #? #? #? ?.
/// ```
fn match_beginning(row: &[State], n: usize) -> bool {
    row[0..n]
        .iter()
        .all(|s| matches!(s, State::Off | State::Unknown))
        && (row.len() == n || matches!(row[n], State::Unknown | State::On))
}

/// Count the number of possible arrangements.
/// ```txt
/// n '#'  > n todo            -> 0
/// n '#?' < n todo            -> 0
/// n todo = 0                 -> 1
/// r[0]   = '.'               -> count(r[1..], ns)
///                            // skip first
/// r[0]   = '#'
///        & row[0..n[0]] = '#?'
///          & n[0] = len(row) -> 1
///                            // exact match
///          & _               -> count(row[n[0]+1..], n[1..])
///                            // continue after first match
///        & _                 -> 0
///                            // doesn't match
/// r[0]   = '?'               -> count(r[1..], ns) + count(['#'] ++ r[1..], ns)
///                            // skip first        + include first
/// ```
fn count(row: &[State], nums: &[usize]) -> usize {
    let total: usize = nums.iter().sum();
    let minimum = row.iter().filter(|x| matches!(x, State::Off)).count();
    let maximum = row
        .iter()
        .filter(|x| matches!(x, State::Off | State::Unknown))
        .count();
    if minimum > total || maximum < total {
        return 0;
    }
    if total == 0 {
        return 1;
    }
    match row[0] {
        State::On => count(&row[1..], nums),
        State::Off => {
            let l = nums[0];
            if match_beginning(row, l) {
                if l == row.len() {
                    1
                } else {
                    count(&row[l + 1..], &nums[1..])
                }
            } else {
                0
            }
        }
        _ => {
            let mut updated = row.to_vec();
            updated[0] = State::Off;
            count(&row[1..], nums) + count(&updated, nums)
        }
    }
}

const EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

#[test]
fn test1() {
    let output = part1(EXAMPLE);
    assert_eq!(output, 21);
}

#[test]
fn test2() {
    let output = part2(EXAMPLE);
    assert_eq!(output, 525152);
}
