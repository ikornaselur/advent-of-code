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

fn search_for_sum(sum: usize, digits: &[usize], with_concat: bool) -> bool {
    if digits.len() == 1 {
        return sum == digits[0];
    }

    let last_digit = digits.last().unwrap();

    // Check if we can divide (for the multiply case)
    if sum % last_digit == 0 {
        let new_sum = sum / last_digit;
        if search_for_sum(new_sum, &digits[..digits.len() - 1], with_concat) {
            return true;
        }
    }

    // Check if we can subtract (for the addition case)
    if sum >= *last_digit {
        let new_sum = sum - last_digit;
        if search_for_sum(new_sum, &digits[..digits.len() - 1], with_concat) {
            return true;
        }

        if with_concat {
            let last_digit_size = 10_usize.pow(last_digit.checked_ilog10().unwrap_or(0) + 1);
            if (sum - last_digit) % last_digit_size == 0 {
                let new_sum = (sum - last_digit) / last_digit_size;
                if search_for_sum(new_sum, &digits[..digits.len() - 1], with_concat) {
                    return true;
                }
            }
        }
    }

    false
}

fn part1(input: &str) -> Result<usize> {
    let rows = parse_input(input)?;

    Ok(rows
        .iter()
        .filter(|(sum, digits)| search_for_sum(*sum, digits, false))
        .map(|(sum, _)| *sum)
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    let rows = parse_input(input)?;

    Ok(rows
        .iter()
        .filter(|(sum, digits)| search_for_sum(*sum, digits, true))
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 11387);
    }

    #[test]
    fn test_search_for_sum_without_concat() {
        // Valid
        assert!(search_for_sum(190, &[10, 19], false));
        assert!(search_for_sum(3267, &[81, 40, 27], false));
        assert!(search_for_sum(292, &[11, 6, 16, 20], false));

        // Invalid
        assert!(!search_for_sum(83, &[17, 5], false));
        assert!(!search_for_sum(156, &[15, 6], false));
        assert!(!search_for_sum(7290, &[6, 8, 6, 15], false));
        assert!(!search_for_sum(161011, &[16, 10, 13], false));
        assert!(!search_for_sum(192, &[17, 8, 14], false));
        assert!(!search_for_sum(21037, &[9, 7, 18, 13], false));
    }

    #[test]
    fn test_search_for_sum_with_concat() {
        // Valid
        assert!(search_for_sum(156, &[15, 6], true));
        assert!(search_for_sum(7290, &[6, 8, 6, 15], true));
        assert!(search_for_sum(192, &[17, 8, 14], true));
    }
}
