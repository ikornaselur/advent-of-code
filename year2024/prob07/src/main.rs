use advent::prelude::*;
use parse::parse_input;

mod parse;

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn search_for_sum(sum: usize, acc: usize, digits: &[usize], with_concat: bool) -> bool {
    // Base case, we ran out of digits to use, have we found the sum?
    if digits.is_empty() {
        return sum == acc;
    }

    // Second case, has the accumulator grown beyond the sum?
    if acc > sum {
        return false;
    }

    let next_digit = digits[0];

    // Try adding next digit
    if search_for_sum(sum, acc + next_digit, &digits[1..], with_concat) {
        return true;
    }

    // Try multiplying next digit
    if search_for_sum(sum, acc * next_digit, &digits[1..], with_concat) {
        return true;
    }

    // If we are in part 2, we just concatenate the next number to the current accumulator
    if with_concat {
        let next_acc =
            acc * 10_usize.pow(next_digit.checked_ilog10().unwrap_or(0) + 1) + next_digit;
        if search_for_sum(sum, next_acc, &digits[1..], with_concat) {
            return true;
        }
    }

    // Couldn't find any!
    false
}

fn part1(input: &str) -> Result<usize> {
    let rows = parse_input(input)?;

    Ok(rows
        .iter()
        .filter(|(sum, digits)| search_for_sum(*sum, 0, digits, false))
        .map(|(sum, _)| *sum)
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    let rows = parse_input(input)?;

    Ok(rows
        .iter()
        .filter(|(sum, digits)| search_for_sum(*sum, 0, digits, true))
        .map(|(sum, _)| *sum)
        .sum())
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
        // Valid
        assert!(search_for_sum(190, 0, &[10, 19], false));
        assert!(search_for_sum(3267, 0, &[81, 40, 27], false));
        assert!(search_for_sum(292, 0, &[11, 6, 16, 20], false));

        // Invalid
        assert!(!search_for_sum(83, 0, &[17, 5], false));
        assert!(!search_for_sum(156, 0, &[15, 6], false));
        assert!(!search_for_sum(7290, 0, &[6, 8, 6, 15], false));
        assert!(!search_for_sum(161011, 0, &[16, 10, 13], false));
        assert!(!search_for_sum(192, 0, &[17, 8, 14], false));
        assert!(!search_for_sum(21037, 0, &[9, 7, 18, 13], false));
    }

    #[test]
    fn test_search_for_sum_with_concat() {
        // Valid
        assert!(search_for_sum(156, 0, &[15, 6], true));
        assert!(search_for_sum(7290, 0, &[6, 8, 6, 15], true));
        assert!(search_for_sum(192, 0, &[17, 8, 14], true));
    }
}
