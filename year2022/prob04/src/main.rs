use advent::prelude::*;

mod parse;
use parse::parse_range_pair;

const INPUT: &str = include_str!("../input.txt");

/// A range of numbers
///
/// Note: The range is inclusive of both the start and the end
#[derive(Debug, PartialEq)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    /// A range is considered to fully contain another range if the start of the
    /// other range is greater than or equal to the start of this range and the
    /// end of the other range is less than or equal to the end of this range.
    fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    input.lines().try_fold(0, |acc, line| {
        let (range1, range2) = parse_range_pair(line)?;
        if range1.fully_contains(&range2) || range2.fully_contains(&range1) {
            Ok(acc + 1)
        } else {
            Ok(acc)
        }
    })
}

fn part2(input: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
