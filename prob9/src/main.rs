use advent_core::error::AdventError;
use advent_core::generic_error;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

struct Sequence {
    numbers: Vec<i64>,
}

impl FromStr for Sequence {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split_whitespace()
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Sequence { numbers })
    }
}

impl Sequence {
    /// Find the next value in the sequence
    ///
    /// This can be done by looking at the difference between the values, then continuing to look
    /// at the difference of those values until we reach all differences being 0.
    /// At that point, we can unwind the stack and add 0 to the previous layer, which will give us
    /// a number to add to the previous layer and so on
    ///
    /// An example is the sequence 0 3 6 9 12 15:
    ///
    /// 0   3   6   9  12  15
    ///   3   3   3   3   3
    ///     0   0   0   0
    ///
    /// The last layer adds a 0, which adds a 3+0 to the layer above, which will add 15+3+0 to the
    /// top layer, so the next value is 18
    fn next_value(&self) -> Result<i64, AdventError> {
        let mut stack = vec![self.numbers.clone()];

        loop {
            let current_layer = stack
                .last_mut()
                .ok_or(generic_error!("Unable to get current layer"))?;

            // Break the loop if all the numbers are 0
            if current_layer.iter().all(|&n| n == 0) {
                break;
            }

            // Add the next layer
            let next_layer = current_layer
                .iter()
                .zip(current_layer.iter().skip(1))
                .map(|(&a, &b)| b - a)
                .collect::<Vec<_>>();

            stack.push(next_layer);
        }

        // Add the last value of all the layers
        Ok(stack.iter().map(|layer| layer.last().unwrap()).sum())
    }
}

fn main() -> Result<(), AdventError> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<i64, AdventError> {
    let sequences = input
        .lines()
        .map(|line| line.parse::<Sequence>())
        .collect::<Result<Vec<_>, _>>()?;
    let sum_of_next_values = sequences
        .iter()
        .map(|sequence| sequence.next_value())
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum::<i64>();
    Ok(sum_of_next_values)
}

fn part2(input: &str) -> Result<i64, AdventError> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_TEST_INPUT: &str = include_str!("../part_1_test.txt");
    const PART_2_TEST_INPUT: &str = include_str!("../part_2_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(PART_1_TEST_INPUT).unwrap(), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART_2_TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_sequence_from_str() {
        let sequence = "1 2 3 4 5".parse::<Sequence>().unwrap();

        assert_eq!(sequence.numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sequence_next_value() {
        assert_eq!(
            "1 2 3 4 5"
                .parse::<Sequence>()
                .unwrap()
                .next_value()
                .unwrap(),
            6
        );
        assert_eq!(
            "1 2 4 7 11"
                .parse::<Sequence>()
                .unwrap()
                .next_value()
                .unwrap(),
            16
        );
        assert_eq!(
            "10 13 16 21 30 45"
                .parse::<Sequence>()
                .unwrap()
                .next_value()
                .unwrap(),
            68
        );
        assert_eq!(
            "0 -1 -2 -3 -4"
                .parse::<Sequence>()
                .unwrap()
                .next_value()
                .unwrap(),
            -5
        )
    }
}
