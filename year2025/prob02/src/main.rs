use advent::prelude::*;
use parse::{IDRange, parse_input};
use std::collections::HashSet;
use std::env;
use std::fs;

mod parse;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        let file_name = &args[1];
        fs::read_to_string(file_name)
            .map_err(|e| error!("Failed to read file {}: {}", file_name, e))?
    } else {
        get_input(2025, 2)?
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

/// Return the first half of a digit
///
/// If the digit is even, like 1234 we return 12
/// If the digit is odd, like 123, we return 1
fn get_left_half_of_digit(digit: u64) -> u64 {
    let digit_count = digit.ilog10() + 1;
    digit / (10u64.pow(digit_count.div_ceil(2)))
}

/// Repeat a digit
///
/// 12 -> 1212
/// 543 -> 543543
fn repeated(digit: u64) -> u64 {
    join_digits(digit, digit)
}

/// Join digits
///
/// join_digits(123, 456) -> 123456
fn join_digits(digit_a: u64, digit_b: u64) -> u64 {
    digit_a * 10u64.pow(digit_b.ilog10() + 1) + digit_b
}

/// Find invalid ids in an id range - part 1
///
/// Take in a range, like 11-22 or 998-1012, and return any values in that range that have
/// sequences of digits repeated twice (like 1010 or 123123)
fn get_invalid_ids_part1(id_range: IDRange) -> Vec<u64> {
    // Take the first half, then increment it until it repeated would be above upper range?
    let start = id_range.start();
    let end = id_range.end();

    // Can't repeat digits to get values less than 10
    if *end < 10 {
        return Vec::new();
    }

    let mut digit = u64::max(1, get_left_half_of_digit(*start));
    let mut invalids = Vec::new();

    loop {
        let r = repeated(digit);
        if r > *end {
            break;
        }

        if r >= *start {
            invalids.push(r);
        }
        digit += 1;
    }

    invalids
}

/// Find invalid ids in an id range - part 2
///
/// A invalid id in part 2 is any number that has at least two repeated sections, this means that
/// both 123123 and 123123123 are invalid
fn get_invalid_ids_part2(id_range: IDRange) -> HashSet<u64> {
    let start = id_range.start();
    let end = id_range.end();

    // Can't repeat digits to get values less than 10
    if *end < 10 {
        return HashSet::new();
    }

    let mut invalids = HashSet::new();

    // Let's start naive.. just with 1 and we repeat until its > end and then continue?
    let mut digit: u64 = 1;
    loop {
        let mut r = repeated(digit);
        if r > *end {
            break;
        }

        // If we're less than start, we just keep adding repeats
        while r < *start {
            r = join_digits(r, digit);
        }
        // And if we are less than end, we'll keep pushing invalids until over
        while r <= *end {
            invalids.insert(r);
            r = join_digits(r, digit);
        }
        digit += 1;
    }

    invalids
}

fn part1(input: &str) -> Result<u64> {
    let ranges: Vec<IDRange> = parse_input(input)?;
    let mut sum = 0;
    for range in ranges {
        sum += get_invalid_ids_part1(range).iter().sum::<u64>();
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<u64> {
    let ranges: Vec<IDRange> = parse_input(input)?;
    let mut sum = 0;
    for range in ranges {
        sum += get_invalid_ids_part2(range).iter().sum::<u64>();
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 1_227_775_554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 4_174_379_265);
    }

    #[test]
    fn test_get_left_half_of_digit() {
        assert_eq!(get_left_half_of_digit(123), 1);
        assert_eq!(get_left_half_of_digit(1234), 12);
        assert_eq!(get_left_half_of_digit(12345), 12);
        assert_eq!(get_left_half_of_digit(123456), 123);
    }

    #[test]
    fn test_repeated() {
        assert_eq!(repeated(123), 123123);
        assert_eq!(repeated(1), 11);
    }

    #[test]
    fn test_get_invalid_ids() {
        assert_eq!(get_invalid_ids_part1(11..=22), vec![11, 22]);
        assert_eq!(
            get_invalid_ids_part1(1_188_511_880..=1_188_511_890),
            vec![1_188_511_885]
        );
    }

    #[test]
    fn test_join_digits() {
        assert_eq!(join_digits(1, 2), 12);
        assert_eq!(join_digits(12, 3), 123);
        assert_eq!(join_digits(1, 23), 123);
    }

    #[test]
    fn test_get_invalid_ids_part2() {
        assert_eq!(get_invalid_ids_part2(11..=22), HashSet::from_iter([11, 22]));
        assert_eq!(
            get_invalid_ids_part2(95..=115),
            HashSet::from_iter([111, 99])
        );
        assert_eq!(
            get_invalid_ids_part2(998..=1012),
            HashSet::from_iter([999, 1010])
        );
        assert_eq!(
            get_invalid_ids_part2(1_188_511_880..=1_188_511_890),
            HashSet::from_iter([1_188_511_885])
        );
        assert_eq!(
            get_invalid_ids_part2(222_220..=222_224),
            HashSet::from_iter([222_222])
        );
    }
}
