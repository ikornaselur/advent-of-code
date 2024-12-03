use advent::prelude::*;
use parse::parse_input;
use std::cmp::Ordering;

mod parse;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let mut values = parse_input(input)?;

    values.sort_unstable();

    let target = 2020;

    let mut left = 0;
    let mut right = values.len() - 1;

    while left < right {
        let sum = values[left] + values[right];
        match sum.cmp(&target) {
            Ordering::Equal => return Ok(values[left] * values[right]),
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
        }
    }

    panic!("No solution found");
}

fn part2(input: &str) -> Result<usize> {
    let mut values = parse_input(input)?;

    let target = 2020;

    values.sort_unstable();

    for i in 0..values.len() {
        let mut left = i + 1;
        let mut right = values.len() - 1;
        while left < right {
            let sum = values[i] + values[left] + values[right];
            match sum.cmp(&target) {
                Ordering::Equal => return Ok(values[i] * values[left] * values[right]),
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
            }
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 514_579);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 241_861_950);
    }
}
