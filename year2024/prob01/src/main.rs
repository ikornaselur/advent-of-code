use advent::prelude::*;
use parse::parse_input;

mod parse;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<i32> {
    let pairs = parse_input(input)?;

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = pairs.into_iter().unzip();
    left.sort_unstable();
    right.sort_unstable();

    let sorted_pairs = left.into_iter().zip(right).collect::<Vec<_>>();

    let sum_of_diffs = sorted_pairs
        .iter()
        .fold(0, |acc, (left, right)| acc + (right - left).abs());

    Ok(sum_of_diffs)
}

fn part2(_input: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
