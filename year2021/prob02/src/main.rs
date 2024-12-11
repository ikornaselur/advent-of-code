use advent::prelude::*;
use parse::parse_input;

mod parse;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    let result = run_with_timeout("Part 1", part1, INPUT)?;
    println!(" > {}", result);

    println!("## Part 2");
    let result = run_with_timeout("Part 2", part2, INPUT)?;
    println!(" > {}", result);

    benchmark_parts(
        |input| {
            part1(input).unwrap();
        },
        |input| {
            part2(input).unwrap();
        },
        INPUT,
    );

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let instructions = parse_input(input)?;

    let (depth, horizontal) = instructions.iter().fold(
        (0, 0),
        |(depth, horizontal), instruction| match instruction {
            (GridDirection::Up, amount) => (depth - amount, horizontal),
            (GridDirection::Down, amount) => (depth + amount, horizontal),
            (GridDirection::Right, amount) => (depth, horizontal + amount),
            _ => panic!("Invalid direction"),
        },
    );

    Ok(depth * horizontal)
}

fn part2(_input: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }
}
