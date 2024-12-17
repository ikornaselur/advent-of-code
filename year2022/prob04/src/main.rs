use advent::prelude::*;

mod parse;
use parse::parse_range_pair;

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

    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    // Note: Not used, because.. I misread the problem first, but it worked!
    // So leaving it in.
    #[allow(dead_code)]
    fn overlap_count(&self, other: &Self) -> u32 {
        if !self.overlaps(other) {
            return 0;
        }

        self.end.min(other.end) - self.start.max(other.start) + 1
    }
}

fn main() -> Result<()> {
    let input = get_input(2022, 4)?;

    println!("## Part 1");
    let result = run_with_timeout("Part 1", part1, &input)?;
    println!(" > {}", result);

    println!("## Part 2");
    let result = run_with_timeout("Part 2", part2, &input)?;
    println!(" > {}", result);

    benchmark_parts(
        |input| {
            part1(input).unwrap();
        },
        |input| {
            part2(input).unwrap();
        },
        &input,
    );

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
    input.lines().try_fold(0, |acc, line| {
        let (range1, range2) = parse_range_pair(line)?;
        if range1.overlaps(&range2) {
            Ok(acc + 1)
        } else {
            Ok(acc)
        }
    })
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 4);
    }

    #[test]
    fn test_range_overlaps() {
        let range1 = Range { start: 1, end: 5 };
        let range2 = Range { start: 5, end: 7 };
        let range3 = Range { start: 6, end: 10 };

        assert!(range1.overlaps(&range2));
        assert!(range2.overlaps(&range1));

        assert!(!range1.overlaps(&range3));
        assert!(!range3.overlaps(&range1));
    }

    #[test]
    fn test_range_overlap_count() {
        let range1 = Range { start: 1, end: 5 };
        let range2 = Range { start: 5, end: 10 };
        let range3 = Range { start: 3, end: 7 };
        let range4 = Range { start: 2, end: 3 };

        assert_eq!(range1.overlap_count(&range2), 1);
        assert_eq!(range2.overlap_count(&range1), 1);

        assert_eq!(range1.overlap_count(&range3), 3);
        assert_eq!(range3.overlap_count(&range1), 3);

        assert_eq!(range2.overlap_count(&range3), 3);
        assert_eq!(range3.overlap_count(&range2), 3);

        assert_eq!(range1.overlap_count(&range4), 2);
        assert_eq!(range4.overlap_count(&range1), 2);
    }
}
