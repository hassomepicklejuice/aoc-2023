use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = part1(&input);
    println!("part1:\t{output}");
    let output = part2(&input);
    println!("part2:\t{output}");
}

const NORTH: &[u8] = b"|7F";
const EAST: &[u8] = b"-7J";
const SOUTH: &[u8] = b"|JL";
const WEST: &[u8] = b"-FL";

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn rev(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
    fn contains(&self, ch: u8) -> bool {
        match self {
            Self::North => NORTH.contains(&ch),
            Self::East => EAST.contains(&ch),
            Self::South => SOUTH.contains(&ch),
            Self::West => WEST.contains(&ch),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Cursor {
    from: Direction,
    pos: (usize, usize),
    curr: u8,
}

fn part1(input: &str) -> usize {
    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let (row, col) = lines
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|c| *c == b'S').map(|col| (row, col)))
        .unwrap();
    let [mut cursor, _] = find_all(row, col, &lines)[..] else {
        panic!("not the right amount of connections!")
    };
    for i in 1.. {
        if cursor.pos == (row, col) {
            return i / 2;
        }
        cursor = find_next(cursor, &lines);
    }
    unreachable!()
}

fn pipe_dirs(ch: u8) -> [Direction; 2] {
    match ch {
        b'L' => [Direction::North, Direction::East],
        b'|' => [Direction::North, Direction::South],
        b'J' => [Direction::North, Direction::West],
        b'F' => [Direction::East, Direction::South],
        b'-' => [Direction::East, Direction::West],
        b'7' => [Direction::South, Direction::West],
        _ => panic!("wrong pipe {ch}"),
    }
}

fn find_all(row: usize, col: usize, map: &[&[u8]]) -> [Cursor; 2] {
    let [one, two] = [
        (Direction::North, row.wrapping_sub(1), col),
        (Direction::East, row, col + 1),
        (Direction::South, row + 1, col),
        (Direction::West, row, col.wrapping_sub(1)),
    ]
    .into_iter()
    .filter_map(
        |(dir, r, c)| match map.get(r).and_then(|line| line.get(c)) {
            Some(pipe) if dir.contains(*pipe) => Some(Cursor {
                from: dir.rev(),
                pos: (r, c),
                curr: *pipe,
            }),
            _ => None,
        },
    )
    .collect::<Vec<_>>()[..] else {
        unreachable!()
    };
    [one, two]
}

fn next_dir(ch: u8, from: Direction) -> Direction {
    match pipe_dirs(ch) {
        [f, s] if f == from => s,
        [f, _] => f,
    }
}

fn find_next(cursor: Cursor, map: &[&[u8]]) -> Cursor {
    let dir = next_dir(cursor.curr, cursor.from);
    let pos = get_coords(dir, cursor.pos.0, cursor.pos.1);
    let curr = map[pos.0][pos.1];
    Cursor {
        from: dir.rev(),
        pos,
        curr,
    }
}

fn get_coords(dir: Direction, row: usize, col: usize) -> (usize, usize) {
    match dir {
        Direction::North => (row.wrapping_sub(1), col),
        Direction::East => (row, col + 1),
        Direction::South => (row + 1, col),
        Direction::West => (row, col.wrapping_sub(1)),
    }
}

const EXAMPLE1: &'static str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

#[test]
fn test1() {
    let output = part1(EXAMPLE1);
    assert_eq!(output, 8);
}

fn part2(input: &str) -> usize {
    let map: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let mut filled = vec![vec![b' '; map[0].len()]; map.len()];
    let (row, col) = map
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|c| *c == b'S').map(|col| (row, col)))
        .unwrap();

    filled[row][col] = b'S';

    let [mut cursor, _] = find_all(row, col, &map)[..] else {
        panic!("not the right amount of connections!")
    };

    while cursor.pos != (row, col) {
        filled[cursor.pos.0][cursor.pos.1] = cursor.curr;
        cursor = find_next(cursor, &map)
    }
    filled
        .iter()
        .for_each(|line| eprintln!("{}", unsafe { std::str::from_utf8_unchecked(line) }));
    for line in filled.iter_mut() {
        let mut pos = b'O';
        for ch in line.iter_mut() {
            match (pos, &ch) {
                (pos, b' ') => *ch = pos,
                (b'O', b'|') => pos = b'I',
                (b'O', b'F') => pos = b'f',
                (b'O', b'L') => pos = b'l',
                (b'I', b'|') => pos = b'O',
                (b'I', b'F') => pos = b'F',
                (b'I', b'L') => pos = b'L',
                (b'f', b'J') => pos = b'I',
                (b'f', b'7') => pos = b'O',
                (b'l', b'7') => pos = b'I',
                (b'l', b'J') => pos = b'O',
                (b'F', b'J') => pos = b'O',
                (b'F', b'7') => pos = b'I',
                (b'L', b'7') => pos = b'O',
                (b'L', b'J') => pos = b'I',
                _ => {}
            }
        }
    }
    eprintln!();
    filled
        .iter()
        .for_each(|line| eprintln!("{}", unsafe { std::str::from_utf8_unchecked(line) }));
    filled
        .into_iter()
        .flatten()
        .filter(|ch| *ch == b'I')
        .count()
}

const EXAMPLE2: &'static str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

#[test]
fn test2() {
    let output = part2(EXAMPLE2);
    assert_eq!(output, 10);
}
