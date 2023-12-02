const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("## Part 1");
    println!(" > {}", part1(INPUT));

    println!("## Part 2");
    println!(" > {}", part2(INPUT));
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

fn part1(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        // Get the left most and right most number in the string
        let numbers = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        acc + numbers.first().unwrap() * 10 + numbers.last().unwrap()
    })
}

fn part2(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let mut numbers: Vec<u32> = Vec::new();
        for (idx, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                numbers.push(char.to_digit(10).unwrap());
                continue;
            }
            for (text, digit) in TEXT_TO_DIGIT_MAP.iter() {
                if line[idx..].starts_with(text) {
                    numbers.push(*digit);
                    break;
                }
            }
        }
        acc + numbers.first().unwrap() * 10 + numbers.last().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_TEST_INPUT: &str = include_str!("../part_1_test.txt");
    const PART_2_TEST_INPUT: &str = include_str!("../part_2_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(PART_1_TEST_INPUT), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART_2_TEST_INPUT), 281);
    }
}
