use advent_core::error::AdventError;
use advent_core::{generic_error, parse_error};
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Condition {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Damaged,
            '.' => Self::Operational,
            '?' => Self::Unknown,
            _ => panic!("Invalid condition: {}", c),
        }
    }
}

#[derive(Debug)]
struct ConditionInfo {
    springs: Vec<Condition>,
    counts: Vec<usize>,
}

impl FromStr for ConditionInfo {
    type Err = AdventError;

    /// Parse a string of the form
    ///
    /// ????.##.##?.. 2,2,3
    ///
    /// which represents the springs conditions on the left and the counts on the right
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let springs: Vec<Condition> = parts
            .next()
            .ok_or(parse_error!("Unable to get springs from line"))?
            .chars()
            .map(Condition::from)
            .collect();

        let counts = parts
            .next()
            .ok_or(parse_error!("Unable to get spring counts from line"))?
            .split(',')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;

        Ok(Self { springs, counts })
    }
}

/// Check if the counts are valid for the conditions
///
/// This functions is just used to check if the counts are valid for a part of the conditions,
/// meaning that if you provide the counts [1, 1, 3] then the following conditions are all valid:
///
///     * #.#.###
///     * #.#.#
///     * #
///
/// because they are valid within the counts
///
/// Returns a tuple of (is_valid_prefix, is_complete_match)
fn counts_valid_prefix(
    conditions: &[Condition],
    counts: &[usize],
) -> Result<(bool, bool), AdventError> {
    let mut current_count = 0;
    let mut count_idx = 0;
    let mut is_counting = false;
    let mut total_damaged = 0;

    for condition in conditions {
        match condition {
            Condition::Damaged => {
                current_count += 1;
                total_damaged += 1;
                is_counting = true;
            }
            Condition::Operational => {
                if is_counting {
                    if count_idx >= counts.len() || current_count != counts[count_idx] {
                        return Ok((false, false));
                    }
                    count_idx += 1;
                    current_count = 0;
                    is_counting = false;
                }
            }
            Condition::Unknown => {
                // We return immediately, as we can't be sure if the count is valid or not
                return Err(generic_error!("Unknown condition"));
            }
        }
    }

    // If we are still counting, check that we are still valid within the next counts
    if is_counting && (count_idx >= counts.len() || current_count > counts[count_idx]) {
        Ok((false, false))
    } else {
        Ok((true, total_damaged == counts.iter().sum::<usize>()))
    }
}

fn backtrack(
    conditions: &mut Vec<Condition>,
    pos: usize,
    counts: &[usize],
    counter: &mut usize,
) -> Result<(), AdventError> {
    if pos >= conditions.len() {
        let (is_valid_prefix, is_complete_match) = counts_valid_prefix(conditions, counts)?;
        if is_valid_prefix && is_complete_match {
            *counter += 1;
        }
        return Ok(());
    }

    match conditions[pos] {
        Condition::Unknown => {
            // Try replacing Unknown with Operational
            conditions[pos] = Condition::Operational;
            let (is_valid_prefix, _) = counts_valid_prefix(&conditions[..pos + 1], counts)?;
            if is_valid_prefix {
                backtrack(conditions, pos + 1, counts, counter)?;
            }

            // Try replacing Unknown with Damaged
            conditions[pos] = Condition::Damaged;
            let (is_valid_prefix, _) = counts_valid_prefix(&conditions[..pos + 1], counts)?;
            if is_valid_prefix {
                backtrack(conditions, pos + 1, counts, counter)?;
            }

            // Reset to Unknown before returning
            conditions[pos] = Condition::Unknown;
        }
        _ => backtrack(conditions, pos + 1, counts, counter)?, // Move to the next position for non-Unknown
    }

    Ok(())
}

fn main() -> Result<(), AdventError> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize, AdventError> {
    let infos: Vec<ConditionInfo> = input
        .lines()
        .map(|l| l.parse::<ConditionInfo>())
        .collect::<Result<Vec<_>, _>>()?;

    let sum_of_options = infos
        .iter()
        .map(|info| {
            let mut conditions = info.springs.clone();
            let mut counter = 0;
            backtrack(&mut conditions, 0, &info.counts, &mut counter).unwrap();
            counter
        })
        .sum::<usize>();

    Ok(sum_of_options)
}

fn part2(input: &str) -> Result<u32, AdventError> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_TEST_INPUT: &str = include_str!("../part_1_test.txt");
    const PART_2_TEST_INPUT: &str = include_str!("../part_2_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(PART_1_TEST_INPUT).unwrap(), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART_2_TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_condition_parsing() {
        let input = "????.##.##?.. 2,2,3";

        let info: ConditionInfo = input.parse().unwrap();

        assert_eq!(
            info.springs,
            vec![
                Condition::Unknown,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Operational,
                Condition::Damaged,
                Condition::Damaged,
                Condition::Operational,
                Condition::Damaged,
                Condition::Damaged,
                Condition::Unknown,
                Condition::Operational,
                Condition::Operational,
            ]
        );
        assert_eq!(info.counts, vec![2, 2, 3]);
    }

    #[test]
    fn test_counts_valid_prefix() {
        let counts = vec![1, 1, 3];

        let (is_valid_prefix, is_complete_match) = counts_valid_prefix(
            "#.#.###"
                .chars()
                .map(Condition::from)
                .collect::<Vec<_>>()
                .as_slice(),
            &counts,
        )
        .unwrap();
        assert!(is_valid_prefix);
        assert!(is_complete_match);

        let (is_valid_prefix, is_complete_match) = counts_valid_prefix(
            "#.#.#"
                .chars()
                .map(Condition::from)
                .collect::<Vec<_>>()
                .as_slice(),
            &counts,
        )
        .unwrap();
        assert!(is_valid_prefix);
        assert!(!is_complete_match);

        let (is_valid_prefix, is_complete_match) = counts_valid_prefix(
            "#".chars()
                .map(Condition::from)
                .collect::<Vec<_>>()
                .as_slice(),
            &counts,
        )
        .unwrap();
        assert!(is_valid_prefix);
        assert!(!is_complete_match);

        let (is_valid_prefix, is_complete_match) = counts_valid_prefix(
            "##".chars()
                .map(Condition::from)
                .collect::<Vec<_>>()
                .as_slice(),
            &counts,
        )
        .unwrap();
        assert!(!is_valid_prefix);
        assert!(!is_complete_match);
    }

    #[test]
    fn test_backtrack_case_1() {
        let mut conditions: Vec<Condition> = "???.##?.".chars().map(Condition::from).collect();
        let counts = vec![1, 1, 3];
        let mut counter = 0;

        backtrack(&mut conditions, 0, &counts, &mut counter).unwrap();

        assert_eq!(counter, 1);
    }

    #[test]
    fn test_backtrack_case_2() {
        let mut conditions: Vec<Condition> =
            ".??..??...?##.".chars().map(Condition::from).collect();
        let counts = vec![1, 1, 3];
        let mut counter = 0;

        backtrack(&mut conditions, 0, &counts, &mut counter).unwrap();

        assert_eq!(counter, 4);
    }

    #[test]
    fn test_backtrack_case_3() {
        let mut conditions: Vec<Condition> = "?###????????".chars().map(Condition::from).collect();
        let counts = vec![3, 2, 1];
        let mut counter = 0;

        backtrack(&mut conditions, 0, &counts, &mut counter).unwrap();

        assert_eq!(counter, 10);
    }
}
