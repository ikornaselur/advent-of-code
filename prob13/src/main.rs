use advent_core::error::AdventError;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Pattern {
    pattern: Vec<Vec<Node>>,
    rows: Vec<u64>,
    cols: Vec<u64>,
}

impl FromStr for Pattern {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = s
            .lines()
            .map(|line| line.chars().map(Node::from).collect::<Vec<Node>>())
            .collect::<Vec<Vec<Node>>>();

        // Since the input is just ash or rocks, we can represent it as bits in a 64 bit binary
        // number
        let rows: Vec<u64> = pattern
            .iter()
            .map(|row| {
                row.iter()
                    .map(|node| match node {
                        Node::Ash => 0,
                        Node::Rock => 1,
                    })
                    .fold(0, |acc, node| (acc << 1) | node)
            })
            .collect();

        let cols: Vec<u64> = (0..pattern[0].len())
            .map(|col| {
                pattern
                    .iter()
                    .map(|row| match row[col] {
                        Node::Ash => 0,
                        Node::Rock => 1,
                    })
                    .fold(0, |acc, node| (acc << 1) | node)
            })
            .collect();
        Ok(Pattern {
            pattern,
            rows,
            cols,
        })
    }
}

impl Pattern {
    fn get_mirror_value(&self, values: &[u64]) -> Option<usize> {
        // Start by finding any pairs of numbers in the rows, by zipping the rows with it self
        // offset by 1
        let pairs: Vec<_> = values
            .iter()
            .zip(values.iter().skip(1))
            .enumerate()
            .filter_map(
                |(i, (row1, row2))| {
                    if row1 == row2 {
                        Some(i)
                    } else {
                        None
                    }
                },
            )
            .collect();

        if pairs.is_empty() {
            return None;
        }

        // Now for each pair, we check if any of them mirror the whole pattern around it
        'outer: for left_id in pairs.iter() {
            let mut left = *left_id;
            let mut right = left + 1;

            loop {
                if values[left] != values[right] {
                    // We found a pair that doesn't match, we can continue onto next pair
                    continue 'outer;
                }
                if left == 0 || right == values.len() - 1 {
                    // We reached the edge of the pattern, without breaking the search, meaning the
                    // pattern is mirrored
                    return Some(left_id + 1);
                }
                left -= 1;
                right += 1;
            }
        }

        None
    }
    /// Find the horizontal mirror row
    ///
    /// If the pattern is not mirrored horizontally, return None
    /// otherwise return the row index where the mirror is
    fn get_horizontal_mirror_value(&self) -> Option<usize> {
        self.get_mirror_value(&self.rows).map(|row| row * 100)
    }

    fn get_vertical_mirror_value(&self) -> Option<usize> {
        self.get_mirror_value(&self.cols)
    }
}

#[derive(Debug, PartialEq)]
enum Node {
    Ash,  // .
    Rock, // #
}

impl From<char> for Node {
    fn from(c: char) -> Self {
        match c {
            '#' => Node::Rock,
            '.' => Node::Ash,
            _ => panic!("Unknown node type"),
        }
    }
}

/// Parse the input of multiple patterns
///
/// Each pattern is separated by a blank line
fn parse_input(input: &str) -> Result<Vec<Pattern>, AdventError> {
    let patterns = input
        .split("\n\n")
        .map(|pattern| pattern.parse::<Pattern>())
        .collect::<Result<Vec<Pattern>, AdventError>>()?;

    Ok(patterns)
}

fn main() -> Result<(), AdventError> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize, AdventError> {
    let patterns: Vec<Pattern> = parse_input(input)?;
    let total = patterns
        .iter()
        .map(|pattern| {
            pattern.get_vertical_mirror_value().unwrap_or(0)
                + pattern.get_horizontal_mirror_value().unwrap_or(0)
        })
        .sum::<usize>();
    Ok(total)
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
        assert_eq!(part1(TEST_INPUT).unwrap(), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_pattern_from_str() {
        let pattern: Pattern = "#.#\n...\n###".parse().unwrap();

        assert_eq!(pattern.pattern.len(), 3);
        assert_eq!(pattern.pattern[0], vec![Node::Rock, Node::Ash, Node::Rock]);
        assert_eq!(pattern.pattern[1], vec![Node::Ash, Node::Ash, Node::Ash]);
        assert_eq!(pattern.pattern[2], vec![Node::Rock, Node::Rock, Node::Rock]);

        assert_eq!(pattern.rows, vec![5, 0, 7]);
        assert_eq!(pattern.cols, vec![5, 1, 5]);
    }

    #[test]
    fn test_parse_input() {
        let patterns = parse_input(TEST_INPUT).unwrap();

        assert_eq!(patterns.len(), 2);
    }

    #[test]
    fn test_get_horizontal_mirror_row() {
        let pattern: Pattern = "#.#\n...\n...\n###".parse().unwrap();
        assert_eq!(pattern.get_horizontal_mirror_value(), None);

        let pattern: Pattern = "#.#\n...\n...\n#.#".parse().unwrap();
        assert_eq!(pattern.get_horizontal_mirror_value(), Some(200));

        let test_patterns: Vec<Pattern> = parse_input(TEST_INPUT).unwrap();

        assert_eq!(test_patterns[0].get_horizontal_mirror_value(), None);
        assert_eq!(test_patterns[1].get_horizontal_mirror_value(), Some(400));
    }

    #[test]
    fn test_get_vertical_mirror_row() {
        let pattern: Pattern = "#..#\n....\n....\n#.##".parse().unwrap();
        assert_eq!(pattern.get_vertical_mirror_value(), None);

        let pattern: Pattern = "#..#\n....\n....\n#..#".parse().unwrap();
        assert_eq!(pattern.get_vertical_mirror_value(), Some(2));

        let test_patterns: Vec<Pattern> = parse_input(TEST_INPUT).unwrap();

        assert_eq!(test_patterns[0].get_vertical_mirror_value(), Some(5));
        assert_eq!(test_patterns[1].get_vertical_mirror_value(), None);
    }
}
