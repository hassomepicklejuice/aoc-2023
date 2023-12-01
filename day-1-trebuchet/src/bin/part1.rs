fn main() {}

fn calibration_value(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .flat_map(|line| {
                let first = line
                    .find(|c: char| c.is_ascii_digit())
                    .and_then(|i| line.get(i..i + 1))?
                    .parse::<usize>()
                    .ok()?;
                let last = line
                    .rfind(|c: char| c.is_ascii_digit())
                    .and_then(|i| line.get(i..i + 1))?
                    .parse::<usize>()
                    .ok()?;
                Some(first * 10 + last)
            })
            .sum(),
    )
}

#[test]
fn example() {
    let input = "1abc2\n\
        pqr3stu8vwx\n\
        a1b2c3d4e5f\n\
        treb7uchet";
    let output = calibration_value(input).expect("142");
    assert_eq!(142, output);
}
