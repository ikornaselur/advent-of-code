use advent::prelude::*;
use parse::parse_input;

mod parse;

fn main() -> Result<()> {
    let input = get_input(2021, 1)?;

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
    let values = parse_input(input)?;
    Ok(values.windows(2).filter(|w| w[0] < w[1]).count())
}

fn part2(input: &str) -> Result<usize> {
    let values = parse_input(input)?;
    Ok(values
        .windows(3)
        .map(|w| w.iter().sum::<usize>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 5);
    }
}
