use advent::prelude::*;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Pattern {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

impl FromStr for Pattern {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();

        // Since the input is just ash or rocks, we can represent it as bits in a 64 bit binary
        // number
        let rows: Vec<u32> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => 0,
                        '#' => 1,
                        _ => panic!("Unknown node type"),
                    })
                    .fold(0, |acc, node| (acc << 1) | node)
            })
            .collect();

        let cols: Vec<u32> = (0..lines[0].len())
            .map(|col| {
                lines
                    .iter()
                    .map(|line| match line.chars().nth(col) {
                        Some('.') => 0,
                        Some('#') => 1,
                        _ => panic!("Unknown node type"),
                    })
                    .fold(0, |acc, node| (acc << 1) | node)
            })
            .collect();
        Ok(Pattern { rows, cols })
    }
}

impl Pattern {
    fn get_mirror_value(&self, values: &[u32], smudges: u32) -> Option<usize> {
        // Start by finding any pairs of numbers in the rows, by zipping the rows with it self
        // offset by 1
        let pairs: Vec<_> = values
            .iter()
            .zip(values.iter().skip(1))
            .enumerate()
            .filter_map(|(i, (row1, row2))| {
                if (row1 ^ row2).count_ones() <= smudges {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        if pairs.is_empty() {
            return None;
        }

        // Now for each pair, we check if any of them mirror the whole pattern around it
        'outer: for left_id in pairs.iter() {
            let mut left = *left_id;
            let mut right = left + 1;

            let mut smudges = smudges;

            loop {
                let ones = (values[left] ^ values[right]).count_ones();
                if smudges > 0 && ones > 0 && ones <= smudges {
                    smudges -= ones;
                } else if ones != 0 {
                    // We found a pair that doesn't match, we can continue onto next pair
                    continue 'outer;
                }
                if left == 0 || right == values.len() - 1 {
                    if smudges > 0 {
                        // We are looking at a pattern that is still smudged
                        continue 'outer;
                    }
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
    fn get_horizontal_mirror_value(&self, smudges: u32) -> Option<usize> {
        self.get_mirror_value(&self.rows, smudges)
            .map(|row| row * 100)
    }

    fn get_vertical_mirror_value(&self, smudges: u32) -> Option<usize> {
        self.get_mirror_value(&self.cols, smudges)
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
            pattern.get_vertical_mirror_value(0).unwrap_or(0)
                + pattern.get_horizontal_mirror_value(0).unwrap_or(0)
        })
        .sum::<usize>();
    Ok(total)
}

fn part2(input: &str) -> Result<usize, AdventError> {
    let patterns: Vec<Pattern> = parse_input(input)?;
    let total = patterns
        .iter()
        .map(|pattern| {
            pattern.get_vertical_mirror_value(1).unwrap_or(0)
                + pattern.get_horizontal_mirror_value(1).unwrap_or(0)
        })
        .sum::<usize>();
    Ok(total)
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
        assert_eq!(part2(TEST_INPUT).unwrap(), 400);
    }

    #[test]
    fn test_pattern_from_str() {
        let pattern: Pattern = "#.#\n...\n###".parse().unwrap();

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
        assert_eq!(pattern.get_horizontal_mirror_value(0), None);

        let pattern: Pattern = "#.#\n...\n...\n#.#".parse().unwrap();
        assert_eq!(pattern.get_horizontal_mirror_value(0), Some(200));

        let test_patterns: Vec<Pattern> = parse_input(TEST_INPUT).unwrap();

        assert_eq!(test_patterns[0].get_horizontal_mirror_value(0), None);
        assert_eq!(test_patterns[1].get_horizontal_mirror_value(0), Some(400));
    }

    #[test]
    fn test_get_vertical_mirror_row() {
        let pattern: Pattern = "#..#\n....\n....\n#.##".parse().unwrap();
        assert_eq!(pattern.get_vertical_mirror_value(0), None);

        let pattern: Pattern = "#..#\n....\n....\n#..#".parse().unwrap();
        assert_eq!(pattern.get_vertical_mirror_value(0), Some(2));

        let test_patterns: Vec<Pattern> = parse_input(TEST_INPUT).unwrap();

        assert_eq!(test_patterns[0].get_vertical_mirror_value(0), Some(5));
        assert_eq!(test_patterns[1].get_vertical_mirror_value(0), None);
    }

    #[test]
    fn test_get_smudged_mirror_row() {
        let test_patterns: Vec<Pattern> = parse_input(TEST_INPUT).unwrap();
        assert_eq!(
            test_patterns[0].get_mirror_value(&test_patterns[0].rows, 1),
            Some(3)
        );
        assert_eq!(
            test_patterns[1].get_mirror_value(&test_patterns[1].rows, 1),
            Some(1)
        );
    }
}
