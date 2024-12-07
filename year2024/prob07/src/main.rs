use advent::prelude::*;
use parse::parse_input;
use std::ops::{Add, Mul};

mod parse;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

type Op<T> = fn(T, T) -> T;

fn search_for_sum(sum: usize, acc: usize, ops: &[Op<usize>], digits: &[usize]) -> bool {
    // Base case, we ran out of digits to use, have we found the sum?
    if digits.is_empty() {
        return sum == acc;
    }

    // Second case, has the accumulator grown beyond the sum?
    // NOTE: This is potentially only valid in part 1 where we are using Addition and
    // Multiplication only
    if acc > sum {
        return false;
    }

    let next_digit = digits[0];

    // Try all the operations
    for op in ops {
        let new_acc = op(acc, next_digit);
        if search_for_sum(sum, new_acc, ops, &digits[1..]) {
            return true;
        }
    }

    // Couldn't find any!
    false
}

fn test_for_mult_add(sum: usize, digits: &[usize]) -> bool {
    let ops: [fn(usize, usize) -> usize; 2] = [Add::add, Mul::mul];

    search_for_sum(sum, 0, &ops, digits)
}

fn part1(input: &str) -> Result<usize> {
    let rows = parse_input(input)?;

    Ok(rows
        .iter()
        .filter(|(sum, digits)| test_for_mult_add(*sum, digits))
        .map(|(sum, _)| *sum)
        .sum())
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
        assert_eq!(part1(TEST_INPUT).unwrap(), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_search_for_sum() {
        let ops: [fn(usize, usize) -> usize; 2] = [Add::add, Mul::mul];

        // Valid
        assert!(search_for_sum(190, 0, &ops, &[10, 19]));
        assert!(search_for_sum(3267, 0, &ops, &[81, 40, 27]));
        assert!(search_for_sum(292, 0, &ops, &[11, 6, 16, 20]));

        // Invalid
        assert!(!search_for_sum(83, 0, &ops, &[17, 5]));
        assert!(!search_for_sum(156, 0, &ops, &[15, 6]));
        assert!(!search_for_sum(7290, 0, &ops, &[6, 8, 6, 15]));
        assert!(!search_for_sum(161011, 0, &ops, &[16, 10, 13]));
        assert!(!search_for_sum(192, 0, &ops, &[17, 8, 14]));
        assert!(!search_for_sum(21037, 0, &ops, &[9, 7, 18, 13]));
    }
}
