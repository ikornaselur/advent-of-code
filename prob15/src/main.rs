use advent_core::error::AdventError;
use advent_core::generic_error;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

struct Element {
    data: String,
    hash: u8,
    label: String,
    label_hash: u8,
    operation: Operation,
    focal_length: Option<u8>,
}

impl FromStr for Element {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hash_fold_func = |acc: u8, c: char| acc.wrapping_add(c as u8).wrapping_mul(17);
        let hash: u8 = s.chars().fold(0, hash_fold_func);

        // The label is all the characters until either - or =
        let label = s
            .chars()
            .take_while(|c| *c != '-' && *c != '=')
            .collect::<String>();
        let label_hash = label.chars().fold(0, hash_fold_func);

        let operation = match s.chars().find(|c| *c == '-' || *c == '=') {
            Some('-') => Operation::Subtract,
            Some('=') => Operation::Equal,
            _ => return Err(AdventError::InvalidInput),
        };

        if operation == Operation::Subtract {
            return Ok(Element {
                data: s.to_owned(),
                hash,
                label,
                label_hash,
                operation,
                focal_length: None,
            });
        }

        let focal_length = s
            .chars()
            .skip_while(|c| *c != '-' && *c != '=')
            .skip(1)
            .collect::<String>()
            .parse::<u8>()?;

        Ok(Element {
            data: s.to_owned(),
            hash,
            label,
            label_hash,
            operation,
            focal_length: Some(focal_length),
        })
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    Equal,
    Subtract,
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

#[derive(Debug, Clone)]
struct Lens {}

fn part2(input: &str) -> Result<usize, AdventError> {
    let elements: Vec<Element> = input
        .trim()
        .split(',')
        .map(|e| e.parse::<Element>())
        .collect::<Result<Vec<Element>, AdventError>>()?;

    let mut boxes: Vec<Vec<(String, u8)>> = vec![Vec::new(); 256];
    for element in elements {
        let idx = element.label_hash as usize;

        let bx = boxes
            .get_mut(idx)
            .ok_or(generic_error!("Invalid box index"))?;

        match element.operation {
            Operation::Equal => {
                let focal_length = element.focal_length.ok_or(generic_error!(
                    "Invalid focal length for element {}",
                    element.data
                ))?;
                let lens = (element.label, focal_length);
                if let Some(i) = bx.iter().position(|e| e.0 == lens.0) {
                    bx[i] = lens;
                } else {
                    bx.push(lens);
                }
            }
            Operation::Subtract => {
                if let Some(i) = bx.iter().position(|e| e.0 == element.label) {
                    bx.remove(i);
                }
            }
        }
    }

    Ok(boxes.iter().enumerate().fold(0, |acc, (box_idx, bx)| {
        acc + bx
            .iter()
            .enumerate()
            .fold(0, |acc, (lens_idx, (_, focal_length))| {
                acc + (box_idx + 1) * (lens_idx + 1) * (*focal_length as usize)
            })
    }))
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 145);
    }

    #[test]
    fn test_element_from_string() {
        let element: Element = "rn=1".parse().unwrap();

        assert_eq!(element.data, "rn=1");
        assert_eq!(element.hash, 30);
        assert_eq!(element.label, "rn");
        assert_eq!(element.label_hash, 0);
        assert_eq!(element.operation, Operation::Equal);
        assert_eq!(element.focal_length, Some(1));
    }
}
