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
    let instructions = parse_input(input)?;

    let (_, pass) = instructions.iter().fold((50, 0), |(pos, pass), value| {
        let next_pos = (pos + value) % 100;
        (next_pos, pass + if next_pos == 0 { 1 } else { 0 })
    });

    Ok(pass)
}

fn part2(input: &str) -> Result<i32> {
    let instructions = parse_input(input)?;

    let (_, clicks) = instructions
        .iter()
        .fold((50i32, 0i32), |(pos, clicks), &v| {
            let next = pos + v;

            let add = if v > 0 {
                next.div_euclid(100) - pos.div_euclid(100)
            } else if v < 0 {
                (pos - 1).div_euclid(100) - (next - 1).div_euclid(100)
            } else {
                0
            };

            (next, clicks + add)
        });

    Ok(clicks)
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 6);
    }
}
