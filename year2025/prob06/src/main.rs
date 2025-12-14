use advent::prelude::*;
use parse::parse_input;
use std::env;
use std::fs;

mod parse;

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Plus,
    Multiply,
}

#[derive(Debug, Clone, PartialEq)]
enum Problem {
    Plus(Vec<u64>),
    Multiply(Vec<u64>),
}

impl Problem {
    fn do_math(&self) -> u64 {
        match self {
            Self::Plus(values) => values.iter().sum(),
            Self::Multiply(values) => values.iter().product(),
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        let file_name = &args[1];
        fs::read_to_string(file_name)
            .map_err(|e| error!("Failed to read file {}: {}", file_name, e))?
    } else {
        get_input(2025, 6)?
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

fn rotate(number_rows: Vec<Vec<u64>>, symbols_row: Vec<Symbol>) -> Vec<Problem> {
    symbols_row
        .iter()
        .enumerate()
        .map(|(idx, symbol)| match &symbol {
            Symbol::Plus => Problem::Plus(number_rows.iter().map(|row| row[idx]).collect()),
            Symbol::Multiply => Problem::Multiply(number_rows.iter().map(|row| row[idx]).collect()),
        })
        .collect()
}

fn part1(input: &str) -> Result<u64> {
    let (number_rows, symbols_row) = parse_input(input)?;
    let out = rotate(number_rows, symbols_row)
        .iter()
        .map(|problem| problem.do_math())
        .sum();
    Ok(out)
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
        assert_eq!(part1(TEST_INPUT).unwrap(), 4_277_556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_rotate() {
        let number_rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let symbols_row = vec![Symbol::Plus, Symbol::Multiply, Symbol::Multiply];

        let rotated = rotate(number_rows, symbols_row);

        assert_eq!(
            rotated,
            vec![
                Problem::Plus(vec![1, 4]),
                Problem::Multiply(vec![2, 5]),
                Problem::Multiply(vec![3, 6]),
            ]
        )
    }

    #[test]
    fn test_problem_do_math() {
        assert_eq!(Problem::Plus(vec![3, 5, 9]).do_math(), 3 + 5 + 9);
        assert_eq!(Problem::Multiply(vec![3, 5, 9]).do_math(), 3 * 5 * 9);
    }
}
