fn main() {
    println!("Hello, world!");
}

fn part1(input: &str) -> usize {
    let patterns = input
        .split("\n\n")
        .map(|pat| pat.lines().map(str::as_bytes).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    todo!()
}

const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

#[test]
fn test1() {
    let output = part1(EXAMPLE);
    assert_eq!(output, 405);
}
