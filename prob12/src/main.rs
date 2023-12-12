use advent_core::error::AdventError;
use advent_core::parse_error;
use rayon::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

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
    conditions: Vec<Condition>,
    conditions_len: usize,
    counts: Vec<usize>,
    cache: HashMap<(usize, usize, usize), usize>,
}

impl ConditionInfo {
    /// Expand the springs and counts
    ///
    /// The springs are just duplicated X times and joined with a new Unknown, so that if we have
    /// #.?# and expand 3 times it will expand to #.?#?#.?#?#.?#
    /// The counts expands similary, without any separator, so that if we have [1, 2] and exapnd 3
    /// times, we'll get [1, 2, 1, 2, 1, 2]
    fn expand(&mut self, times: usize) {
        let mut new_conditions: Vec<Condition> = Vec::new();
        let mut new_counts: Vec<usize> = Vec::new();

        for _ in 0..(times - 1) {
            new_conditions.extend(self.conditions.clone());
            new_conditions.push(Condition::Unknown);
            new_counts.extend(self.counts.iter());
        }
        new_conditions.extend(self.conditions.clone());
        new_counts.extend(self.counts.iter());

        self.conditions = new_conditions;
        self.conditions_len = self.conditions.len();
        self.counts = new_counts;
    }

    fn backtrack(
        &mut self,
        pos: usize,
        counts_idx: usize,
        current_damage_count: usize,
    ) -> Result<usize, AdventError> {
        if pos >= self.conditions_len {
            // The case of ending on a non-damaged spring
            if counts_idx == self.counts.len() && current_damage_count == 0 {
                return Ok(1);
            }
            // The case of ending on a damaged spring
            if counts_idx == self.counts.len() - 1
                && current_damage_count == self.counts[counts_idx]
            {
                return Ok(1);
            }
            return Ok(0);
        }

        let out = match self.conditions[pos] {
            Condition::Unknown => {
                // Try replacing Unknown with Operational
                self.conditions[pos] = Condition::Operational;
                let if_conditional = self.backtrack(pos, counts_idx, current_damage_count)?;

                // Try replacing Unknown with Damaged
                self.conditions[pos] = Condition::Damaged;
                let if_damaged = self.backtrack(pos, counts_idx, current_damage_count)?;

                // Reset to Unknown before returning
                self.conditions[pos] = Condition::Unknown;

                if_conditional + if_damaged
            }
            Condition::Damaged => {
                if counts_idx >= self.counts.len()
                    || current_damage_count + 1 > self.counts[counts_idx]
                {
                    // We can't have more damaged than the count allows
                    return Ok(0);
                }

                self.backtrack(pos + 1, counts_idx, current_damage_count + 1)?
            }
            Condition::Operational => {
                if current_damage_count == 0 {
                    self.backtrack(pos + 1, counts_idx, 0)?
                } else if current_damage_count > 0
                    && counts_idx < self.counts.len()
                    && self.counts[counts_idx] == current_damage_count
                {
                    self.backtrack(pos + 1, counts_idx + 1, 0)?
                } else {
                    0
                }
            }
        };

        Ok(out)
    }
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
        let conditions: Vec<Condition> = parts
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
        let conditions_len = conditions.len();

        Ok(Self {
            conditions,
            conditions_len,
            counts,
            cache: HashMap::new(),
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

fn part1(input: &str) -> Result<usize, AdventError> {
    let mut infos: Vec<ConditionInfo> = input
        .lines()
        .map(|l| l.parse::<ConditionInfo>())
        .collect::<Result<Vec<_>, _>>()?;

    let sum_of_options = infos
        .iter_mut()
        .map(|info| info.backtrack(0, 0, 0).unwrap())
        .sum::<usize>();

    Ok(sum_of_options)
}

fn part2(input: &str) -> Result<usize, AdventError> {
    let mut infos: Vec<ConditionInfo> = input
        .lines()
        .map(|l| l.parse::<ConditionInfo>())
        .collect::<Result<Vec<_>, _>>()?;

    // Expand all infos by 5
    infos.iter_mut().for_each(|info| info.expand(5));

    let total = infos.len();
    let finished = Arc::new(Mutex::new(0));

    let sum_of_options = infos
        //.par_iter_mut()
        .iter_mut()
        .map(|info| {
            let counter = info.backtrack(0, 0, 0).unwrap();
            let mut finished = finished.lock().unwrap();
            *finished += 1;
            println!("Finished: {}/{}", *finished, total);
            counter
        })
        .sum::<usize>();

    Ok(sum_of_options)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 525152);
    }

    #[test]
    fn test_condition_parsing() {
        let input = "????.##.##?.. 2,2,3";

        let info: ConditionInfo = input.parse().unwrap();

        assert_eq!(
            info.conditions,
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
    fn test_backtrack_case_1() {
        let mut info: ConditionInfo = "???.##?. 1,1,3".parse().unwrap();
        let counter = info.backtrack(0, 0, 0).unwrap();

        assert_eq!(counter, 1);
    }

    #[test]
    fn test_backtrack_case_2() {
        let mut info: ConditionInfo = ".??..??...?##. 1,1,3".parse().unwrap();
        let counter = info.backtrack(0, 0, 0).unwrap();

        assert_eq!(counter, 4);
    }

    #[test]
    fn test_backtrack_case_3() {
        let mut info: ConditionInfo = "?###???????? 3,2,1".parse().unwrap();
        let counter = info.backtrack(0, 0, 0).unwrap();

        assert_eq!(counter, 10);
    }

    #[test]
    fn test_condition_info_expand() {
        let mut info: ConditionInfo = ".# 1".parse().unwrap();

        info.expand(2);

        assert_eq!(
            info.conditions,
            vec![
                Condition::Operational,
                Condition::Damaged,
                Condition::Unknown,
                Condition::Operational,
                Condition::Damaged,
            ]
        );
        assert_eq!(info.counts, vec![1, 1]);
    }
}
