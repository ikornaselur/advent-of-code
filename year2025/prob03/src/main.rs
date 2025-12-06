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

fn get_max_joltage(batteries: &[u32]) -> u32 {
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

fn part1(input: &str) -> Result<u32> {
    let digit_rows = parse_input(input)?;
    let total_joltage = digit_rows
        .iter()
        .map(|row| get_max_joltage(row))
        .sum::<u32>();

    Ok(total_joltage)
}

fn part2(_input: &str) -> Result<usize> {
    // let thing = parse_input(input)?;
    Ok(0)
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_get_max_joltage() {
        assert_eq!(get_max_joltage(&[1, 5, 4, 2, 5, 9, 4, 3, 4, 5, 9]), 99);
        assert_eq!(get_max_joltage(&[1, 5, 4, 2, 5, 9, 4, 3, 4, 5, 8]), 98);
        assert_eq!(get_max_joltage(&[1, 5, 4, 2, 5, 8, 4, 3, 4, 5, 9]), 89);
        assert_eq!(get_max_joltage(&[1, 5, 4, 2, 5, 8, 4, 3, 4, 5, 8]), 88);
    }
}
