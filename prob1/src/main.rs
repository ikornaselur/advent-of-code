fn main() {
    let input = include_str!("../input.txt");

    println!("# Problem 1");

    println!("## Part 1");
    println!(" > {}", part1(input));

    println!("## Part 2");
    println!(" > {}", part2(input));
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
            .collect::<Vec<char>>();
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        let value = format!("{}{}", first, last).parse::<u32>().unwrap();
        acc + value
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
        // Get the left most and right most number in the string
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        let value = format!("{}{}", first, last).parse::<u32>().unwrap();
        acc + value
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";

        println!("input: {}", input);
        assert_eq!(part2(input), 281);
    }
}
