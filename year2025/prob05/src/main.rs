use advent::prelude::*;
use parse::parse_input;
use std::env;
use std::fs;
use std::ops::RangeInclusive;

mod parse;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        let file_name = &args[1];
        fs::read_to_string(file_name)
            .map_err(|e| error!("Failed to read file {}: {}", file_name, e))?
    } else {
        get_input(2025, 1)?
    };

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

fn part1(input: &str) -> Result<usize> {
    let (fresh_ranges, ingredient_ids) = parse_input(input)?;

    let joined_ranges = prepare_ranges(fresh_ranges)?;

    // This should be easy with just a naive check.. I'm curious what part 2 will be though.
    // Definitely will have to join ranges that overlap for example
    let count = ingredient_ids.iter().fold(0, |acc, ingredient_id| {
        acc + if joined_ranges
            .iter()
            .any(|range| range.contains(ingredient_id))
        {
            1
        } else {
            0
        }
    });

    Ok(count)
}

fn is_overlapping<T: Copy + Ord>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}

/// Join two ranges
///
/// Note: There are no checks that they overlap!
fn join_ranges<T: Copy + Ord>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> RangeInclusive<T> {
    RangeInclusive::new(*a.start().min(b.start()), *a.end().max(b.end()))
}

fn prepare_ranges<T: Copy + Ord>(ranges: Vec<RangeInclusive<T>>) -> Result<Vec<RangeInclusive<T>>> {
    let mut ranges = ranges;

    // Let's sort by the start
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    // Combine all overlapping ranges, then just sum up?
    let mut joined_ranges = Vec::new();

    let mut current_range = ranges.first().unwrap().clone();

    for fresh_range in ranges.iter().skip(1) {
        if is_overlapping(&current_range, fresh_range) {
            current_range = join_ranges(&current_range, fresh_range);
        } else {
            joined_ranges.push(current_range);
            current_range = fresh_range.clone();
        }
    }

    joined_ranges.push(current_range);

    Ok(joined_ranges)
}

fn part2(input: &str) -> Result<u64> {
    let (fresh_ranges, _) = parse_input(input)?;

    let joined_ranges = prepare_ranges(fresh_ranges)?;

    let count = joined_ranges
        .iter()
        .fold(0, |acc, range| acc + (range.end() - range.start() + 1));

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 14);
    }

    #[test]
    fn test_is_overlapping() {
        assert!(is_overlapping(&(1..=5), &(5..=10)));
        assert!(is_overlapping(&(1..=6), &(5..=10)));
        assert!(is_overlapping(&(4..=6), &(1..=10)));
        assert!(is_overlapping(&(4..=6), &(1..=4)));
        assert!(is_overlapping(&(4..=6), &(1..=5)));

        assert!(!is_overlapping(&(1..=3), &(4..=5)));
        assert!(!is_overlapping(&(4..=5), &(1..=3)));
    }

    #[test]
    fn test_join_ranges() {
        assert_eq!(join_ranges(&(1..=5), &(5..=10)), 1..=10);
        assert_eq!(join_ranges(&(1..=8), &(5..=10)), 1..=10);
        assert_eq!(join_ranges(&(1..=8), &(1..=10)), 1..=10);
        assert_eq!(join_ranges(&(4..=8), &(1..=10)), 1..=10);
    }
}
