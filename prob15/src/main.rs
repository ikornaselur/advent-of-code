use advent_core::error::AdventError;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

struct Element {
    data: String,
    hash: u8,
}

impl FromStr for Element {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hash: u8 = s
            .chars()
            .fold(0, |acc, c| acc.wrapping_add(c as u8).wrapping_mul(17));

        Ok(Element {
            data: s.to_owned(),
            hash,
        })
    }
}

fn main() -> Result<(), AdventError> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<u32, AdventError> {
    let elements: Vec<Element> = input
        .trim()
        .split(',')
        .map(|e| e.parse::<Element>())
        .collect::<Result<Vec<Element>, AdventError>>()?;

    let sum = elements.iter().fold(0, |acc, e| acc + e.hash as u32);

    Ok(sum)
}

fn part2(input: &str) -> Result<u32, AdventError> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_element_from_string() {
        let element: Element = "HASH".parse().unwrap();

        assert_eq!(element.data, "HASH");
        assert_eq!(element.hash, 52);
    }
}
