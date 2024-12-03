use advent::prelude::*;
use parse::parse_input;

mod parse;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

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
