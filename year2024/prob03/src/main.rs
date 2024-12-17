use advent::prelude::*;
use parse::parse_input;

mod parse;

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

fn main() -> Result<()> {
    let input = get_input(2024, 3)?;

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
    Ok(instructions
        .into_iter()
        .map(|ins| match ins {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    let instructions = parse_input(input)?;

    Ok(instructions
        .into_iter()
        .scan(true, |enabled, instruction| match instruction {
            Instruction::Mul(a, b) if *enabled => Some(a * b),
            Instruction::Dont => {
                *enabled = false;
                Some(0)
            }
            Instruction::Do => {
                *enabled = true;
                Some(0)
            }
            _ => Some(0),
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");
    const TEST2_INPUT: &str = include_str!("../test2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST2_INPUT).unwrap(), 48);
    }
}
