fn main() {
    println!("Hello, world!");
}

fn part1(input: &str) -> usize {
    let rows: Vec<_> = input
        .lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(row, nums)| {
            (
                row.as_bytes().to_vec(),
                nums.split(',')
                    .flat_map(str::parse::<usize>)
                    .collect::<Vec<usize>>(),
            )
        })
        .collect();

    rows.into_iter()
        .map(|(row, nums)| find_arrangements(&row, &nums))
        .sum()
}

fn find_arrangements(row: &[u8], nums: &[usize]) -> usize {
    todo!()
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
