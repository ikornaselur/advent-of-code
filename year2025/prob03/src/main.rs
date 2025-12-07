use advent::prelude::*;
use parse::parse_input;
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
        get_input(2025, 3)?
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

fn get_max_joltage_part1(batteries: &[u32]) -> u32 {
    // Let's get counts of digits to get theoretical max
    // We'll skip counting the last value, for reasons that might become clear later
    let counts =
        batteries[..batteries.len().saturating_sub(1)]
            .iter()
            .fold(HashMap::new(), |mut acc, x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            });

    let mut max_joltage = 0;

    for val in [9, 8, 7, 6, 5, 4, 3, 2, 1] {
        let val_count = *counts.get(&val).unwrap_or(&0);

        // If we have none of these, we just skip
        if val_count == 0 {
            continue;
        }

        // If we have at least one, then lets find the earliest, and from there we find the _max_
        // value on the right of it, if that's higher than max joltage, we set it and continue
        let first_val_idx = batteries.iter().position(|&x| x == val).unwrap();

        if first_val_idx == batteries.len() - 1 {
            panic!("At the end, deal with it");
        }
        let joltage = val * 10 + batteries[first_val_idx + 1..].iter().max().unwrap();
        if joltage > max_joltage {
            max_joltage = joltage;
        }
    }

    max_joltage
}

/// Get max joltage - recurise (maybe?)
///
/// Hypothesis 1: If we have ANY nines, and we can get a `count` digit number, then the highest
/// joltage will start with nine. We might have to compare multiple numbers that each start with
/// a nine.. if that works, then we can figure out how to optimise later
///
/// Hypothesis 2: If we have two nines, then the left nine should always give you the highest
/// joltage, because the left nine can always create the number that the right nine would
/// create. This means that we only ever have to recurse down with the highest value, once?
///
/// Hypothesis 3: We just store earliest occurance of digits, instead of counts, if Hypothesis 2 is
/// correct
fn get_max_joltage_recursive(batteries: &[u32], count: usize) -> u64 {
    if count == 0 {
        panic!("Shouldn't call with 0");
    }

    // We're skipping `count - 1` digits at the end, because if we were to use those, there
    // wouldn't be any digits left to recurse with
    let earliest_occurrance = batteries[..batteries.len().saturating_sub(count - 1)]
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (idx, x)| {
            if !acc.contains_key(x) {
                acc.insert(x, idx);
            }
            acc
        });
    // Then it's time to test with the highest values.
    for digit in [9, 8, 7, 6, 5, 4, 3, 2, 1] {
        if let Some(digit_idx) = earliest_occurrance.get(&digit) {
            if count == 1 {
                return digit as u64;
            }
            let rest = get_max_joltage_recursive(&batteries[*digit_idx + 1..], count - 1);
            return digit as u64 * (10u64.pow((count - 1) as u32)) + rest;
        }
    }

    panic!("Shouldn't be reachable!");
}

fn part1(input: &str) -> Result<u32> {
    let digit_rows = parse_input(input)?;
    let total_joltage = digit_rows
        .iter()
        .map(|row| get_max_joltage_part1(row))
        .sum::<u32>();

    Ok(total_joltage)
}

fn part2(input: &str) -> Result<u64> {
    let digit_rows = parse_input(input)?;
    let total_joltage = digit_rows
        .iter()
        .map(|row| get_max_joltage_recursive(row, 12))
        .sum::<u64>();

    Ok(total_joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 3_121_910_778_619);
    }

    #[test]
    fn test_get_max_joltage() {
        assert_eq!(
            get_max_joltage_part1(&[1, 5, 4, 2, 5, 9, 4, 3, 4, 5, 9]),
            99
        );
        assert_eq!(
            get_max_joltage_part1(&[1, 5, 4, 2, 5, 9, 4, 3, 4, 5, 8]),
            98
        );
        assert_eq!(
            get_max_joltage_part1(&[1, 5, 4, 2, 5, 8, 4, 3, 4, 5, 9]),
            89
        );
        assert_eq!(
            get_max_joltage_part1(&[1, 5, 4, 2, 5, 8, 4, 3, 4, 5, 8]),
            88
        );
    }

    #[test]
    fn test_get_max_joltage_recursive() {
        assert_eq!(
            get_max_joltage_recursive(&[1, 5, 4, 2, 5, 9, 4, 3, 4, 5, 9], 2),
            99
        );
        assert_eq!(
            get_max_joltage_recursive(&[1, 5, 4, 2, 5, 9, 4, 3, 4, 5, 8], 2),
            98
        );
        assert_eq!(
            get_max_joltage_recursive(&[1, 5, 4, 2, 5, 8, 4, 3, 4, 5, 9], 2),
            89
        );
        assert_eq!(
            get_max_joltage_recursive(&[1, 5, 4, 2, 5, 8, 4, 3, 4, 5, 8], 2),
            88
        );
        assert_eq!(
            get_max_joltage_recursive(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12),
            987_654_321_111
        );
        assert_eq!(
            get_max_joltage_recursive(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12),
            434_234_234_278
        );
    }
}
