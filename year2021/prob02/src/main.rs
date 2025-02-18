use advent::prelude::*;
use parse::parse_input;

mod parse;

fn main() -> Result<()> {
    let input = get_input(2021, 2)?;

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

fn part2(input: &str) -> Result<usize> {
    let instructions = parse_input(input)?;

    let (depth, horizontal, _) =
        instructions
            .iter()
            .fold(
                (0, 0, 0),
                |(depth, horizontal, aim), instruction| match instruction {
                    (GridDirection::Up, amount) => (depth, horizontal, aim - amount),
                    (GridDirection::Down, amount) => (depth, horizontal, aim + amount),
                    (GridDirection::Right, amount) => {
                        (depth + aim * amount, horizontal + amount, aim)
                    }
                    _ => panic!("Invalid direction"),
                },
            );

    Ok(depth * horizontal)
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 900);
    }
}
