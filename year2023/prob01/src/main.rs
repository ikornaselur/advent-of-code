use advent::prelude::*;

fn main() -> Result<()> {
    let input = get_input(2023, 1)?;

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

// Create a text to digit map, converting digits "one" to "nine" to 1 to 9
const TEXT_TO_DIGIT_MAP: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn part1(input: &str) -> Result<u32> {
    input.lines().try_fold(0, |acc, line| {
        // Get the left most and right most number in the string
        let numbers: Result<Vec<u32>> = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).ok_or(AdventError::ConversionError))
            .collect();

        let numbers = numbers?;

        Ok(acc
            + numbers
                .first()
                .ok_or(AdventError::NoNumbers)?
                .saturating_mul(10)
            + numbers.last().ok_or(AdventError::NoNumbers)?)
    })
}

fn part2(input: &str) -> Result<u32> {
    input.lines().try_fold(0, |acc, line| {
        let mut numbers: Vec<u32> = Vec::new();
        for (idx, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                numbers.push(char.to_digit(10).ok_or(AdventError::ConversionError)?);
                continue;
            }
            for (text, digit) in TEXT_TO_DIGIT_MAP.iter() {
                if line[idx..].starts_with(text) {
                    numbers.push(*digit);
                    break;
                }
            }
        }
        Ok(acc
            + numbers
                .first()
                .ok_or(AdventError::NoNumbers)?
                .saturating_mul(10)
            + numbers.last().ok_or(AdventError::NoNumbers)?)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_TEST_INPUT: &str = include_str!("../part_1_test.txt");
    const PART_2_TEST_INPUT: &str = include_str!("../part_2_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(PART_1_TEST_INPUT).unwrap(), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART_2_TEST_INPUT).unwrap(), 281);
    }
}
