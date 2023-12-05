use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let output = part1(&input)?;
    println!("part1: {output}");
    let output = part2(&input)?;
    println!("part2: {output}");
    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .flat_map(|line| {
            let (first, last) = get_first_last_digit(line)?;
            Some(first * 10 + last)
        })
        .sum())
}

fn get_first_last_digit(line: &str) -> Option<(usize, usize)> {
    let mut iter = line.chars().filter(char::is_ascii_digit);
    let first = iter.next().and_then(|c| c.to_digit(10))?;
    let last = iter
        .next_back()
        .and_then(|c| c.to_digit(10))
        .unwrap_or(first);
    Some((first as usize, last as usize))
}

fn part2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .flat_map(|line| {
            let (first, last) = get_first_last_words(line)?;
            Some(first * 10 + last)
        })
        .sum())
}

fn get_digit(s: &str) -> Option<usize> {
    match s.as_bytes().get(0)? {
        b'1' => Some(1),
        b'2' => Some(2),
        b'3' => Some(3),
        b'4' => Some(4),
        b'5' => Some(5),
        b'6' => Some(6),
        b'7' => Some(7),
        b'8' => Some(8),
        b'9' => Some(9),
        b'o' if s.starts_with("one") => Some(1),
        b'e' if s.starts_with("eight") => Some(8),
        b'n' if s.starts_with("nine") => Some(9),
        b't' => {
            if s.starts_with("two") {
                Some(2)
            } else if s.starts_with("three") {
                Some(3)
            } else {
                None
            }
        }
        b'f' => {
            if s.starts_with("four") {
                Some(4)
            } else if s.starts_with("five") {
                Some(5)
            } else {
                None
            }
        }
        b's' => {
            if s.starts_with("six") {
                Some(6)
            } else if s.starts_with("seven") {
                Some(7)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn get_first_last_words(line: &str) -> Option<(usize, usize)> {
    let first = line
        .char_indices()
        .map(|(pos, _)| &line[pos..])
        .find_map(get_digit)?;
    let last = line
        .char_indices()
        .map(|(pos, _)| &line[pos..])
        .rev()
        .find_map(get_digit)?;

    Some((first, last))
}

#[test]
fn test1() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let output = part1(input).expect("142");
    assert_eq!(142, output);
}

#[test]
fn test2() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let output = part2(input).expect("281");
    assert_eq!(281, output);
}
