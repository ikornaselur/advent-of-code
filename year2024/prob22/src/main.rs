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
        get_input(2024, 22)?
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
    let digits = parse_input(input)?;

    Ok(digits
        .iter()
        .map(|digit| {
            let mut num = *digit;
            for _ in 0..2000 {
                num = steps(num);
            }
            num
        })
        .sum())
}

fn part2(_input: &str) -> Result<usize> {
    // let thing = parse_input(input)?;
    Ok(0)
}

fn step1(num: usize) -> usize {
    ((num * 64) ^ num) % 16_777_216
}

fn step2(num: usize) -> usize {
    ((num / 32) ^ num) % 16_777_216
}

fn step3(num: usize) -> usize {
    ((num * 2048) ^ num) % 16_777_216
}

fn steps(num: usize) -> usize {
    step3(step2(step1(num)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 37_327_623);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_steps() {
        assert_eq!(steps(123), 15_887_950);
    }
}
