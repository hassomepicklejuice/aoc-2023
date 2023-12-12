fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let output = part1(&input);
    println!("part1:\t{output}");
    let output = part2(&input, 1000000);
    println!("part2:\t{output}");
}

fn part1(input: &str) -> usize {
    let lines: Vec<Vec<u8>> = input
        .lines()
        .flat_map(|line| {
            if line.as_bytes().iter().all(|ch| *ch == b'.') {
                Vec::from([line.as_bytes().to_vec(), line.as_bytes().to_vec()])
            } else {
                Vec::from([line.as_bytes().to_vec()])
            }
        })
        .collect();
    let cols: Vec<Vec<u8>> = transpose(lines)
        .into_iter()
        .flat_map(|col| {
            if col.iter().all(|ch| *ch == b'.') {
                Vec::from([col.clone(), col])
            } else {
                Vec::from([col])
            }
        })
        .collect();
    let lines = transpose(cols);

    let galaxies: Vec<(usize, usize)> = lines
        .into_iter()
        .enumerate()
        .flat_map(|(r, line)| {
            line.into_iter()
                .enumerate()
                .filter_map(move |(c, ch)| if ch == b'#' { Some((r, c)) } else { None })
        })
        .collect();

    let mut distances = vec![];
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies[(i + 1)..].iter() {
            let diff = g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1);
            distances.push(diff);
        }
    }
    distances.into_iter().sum()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        return v;
    }
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect())
        .collect()
}

fn pretty(v: &[Vec<u8>]) {
    v.iter()
        .for_each(|line| eprintln!("{}", unsafe { std::str::from_utf8_unchecked(&line) }));
    eprintln!();
}

fn part2(input: &str, expansion: usize) -> usize {
    let mut galaxies: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.char_indices()
                .filter_map(move |(c, ch)| if ch == '#' { Some((r, c)) } else { None })
        })
        .collect();

    let rows: Vec<usize> = input
        .lines()
        .enumerate()
        .filter_map(|(r, line)| {
            if line.chars().all(|ch| ch == '.') {
                Some(r)
            } else {
                None
            }
        })
        .collect();
    let cols: Vec<usize> = (0..input.lines().next().unwrap().len())
        .into_iter()
        .filter_map(|c| {
            if input
                .lines()
                .map(|line| line.as_bytes()[c])
                .all(|ch| ch == b'.')
            {
                Some(c)
            } else {
                None
            }
        })
        .collect();

    for row in rows.into_iter().rev() {
        galaxies.iter_mut().for_each(|(r, _c)| {
            if *r > row {
                *r += expansion - 1;
            }
        });
    }
    for col in cols.into_iter().rev() {
        galaxies.iter_mut().for_each(|(_r, c)| {
            if *c > col {
                *c += expansion - 1;
            }
        });
    }

    let mut distances = vec![];
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies[(i + 1)..].iter() {
            let diff = g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1);
            distances.push(diff);
        }
    }
    distances.into_iter().sum()
}

const EXAMPLE: &'static str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

#[test]
fn test1() {
    let output = part1(EXAMPLE);
    assert_eq!(output, 374);
}

#[test]
fn test2() {
    let output = part2(EXAMPLE, 100);
    assert_eq!(output, 8410);
}
