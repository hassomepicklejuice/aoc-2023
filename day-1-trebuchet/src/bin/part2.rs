use std::{
    error::Error,
    io::{stdin, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let output = real_calibration_value(&input).ok_or("No calibration value")?;
    println!("{output}");
    Ok(())
}

fn real_calibration_value(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .flat_map(|line| {
                let first = find_first(line)?;
                let last = find_last(line)?;
                Some(first * 10 + last)
            })
            .sum(),
    )
}

fn find_first(input: &str) -> Option<usize> {
    let mut pos = input.find(|c: char| c.is_ascii_digit());
    let mut digit: Option<usize> = pos
        .and_then(|i| input.get(i..i + 1))
        .and_then(|s| s.parse().ok());
    for (i, num) in [
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ]
    .into_iter()
    {
        if let Some(p) = input.find(num) {
            match pos {
                Some(n) if p < n => {
                    pos = Some(p);
                    digit = Some(i);
                }
                None => {
                    pos = Some(p);
                    digit = Some(i);
                }
                _ => {}
            }
        }
    }
    digit
}

fn find_last(input: &str) -> Option<usize> {
    let mut pos = input.rfind(|c: char| c.is_ascii_digit());
    let mut digit: Option<usize> = pos
        .and_then(|i| input.get(i..i + 1))
        .and_then(|s| s.parse().ok());
    for (i, num) in [
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ]
    .into_iter()
    {
        if let Some(p) = input.rfind(num) {
            match pos {
                Some(n) if p > n => {
                    pos = Some(p);
                    digit = Some(i);
                }
                None => {
                    pos = Some(p);
                    digit = Some(i);
                }
                _ => {}
            }
        }
    }
    digit
}

#[test]
fn example() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let output = real_calibration_value(input).expect("281");
    assert_eq!(281, output);
}
