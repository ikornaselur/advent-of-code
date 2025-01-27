use advent::prelude::*;

struct Sequence {
    stack: Vec<Vec<i64>>,
}

impl FromStr for Sequence {
    type Err = AdventError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let numbers = s
            .split_whitespace()
            .map(|s| s.parse::<i64>())
            .collect::<std::result::Result<Vec<_>, _>>()?;

        let mut stack = vec![numbers];

        loop {
            let current_layer = stack
                .last_mut()
                .ok_or(error!("Unable to get current layer"))?;

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

        Ok(Sequence { stack })
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
    fn next_value(&self) -> Result<i64> {
        // Add the last value of all the layers
        Ok(self.stack.iter().map(|layer| layer.last().unwrap()).sum())
    }

    /// Find the previous value in the sequence
    ///
    /// This is basically the opposite of 'next_value', by finding what number would be before the
    /// current sequence
    ///
    /// For the example above, with 0 3 6 9 12 15 we'd be looking for -3 at the start
    fn previous_value(&self) -> Result<i64> {
        let mut last = 0;
        for layer in self.stack.iter().rev() {
            let first_val = layer.first().ok_or(error!("Unable to get first value"))?;
            last = first_val - last;
        }

        Ok(last)
    }
}

fn main() -> Result<()> {
    let input = get_input(2023, 9)?;

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

fn part1(input: &str) -> Result<i64> {
    let sequences: Vec<Sequence> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>>>()?;
    let sum_of_next_values = sequences
        .iter()
        .map(|sequence| sequence.next_value())
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum::<i64>();
    Ok(sum_of_next_values)
}

fn part2(input: &str) -> Result<i64> {
    let sequences: Vec<Sequence> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>>>()?;
    let sum_of_previous_values = sequences
        .iter()
        .map(|sequence| sequence.previous_value())
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum::<i64>();
    Ok(sum_of_previous_values)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 2);
    }

    #[test]
    fn test_sequence_from_str() {
        let sequence = "1 2 3 4 5".parse::<Sequence>().unwrap();

        assert_eq!(sequence.stack[0], vec![1, 2, 3, 4, 5]);
        assert_eq!(sequence.stack[1], vec![1, 1, 1, 1]);
        assert_eq!(sequence.stack[2], vec![0, 0, 0]);
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

    #[test]
    fn test_sequence_previous_value() {
        assert_eq!(
            "1 2 3 4 5"
                .parse::<Sequence>()
                .unwrap()
                .previous_value()
                .unwrap(),
            0
        );
        assert_eq!(
            "10 13 16 21 30 45"
                .parse::<Sequence>()
                .unwrap()
                .previous_value()
                .unwrap(),
            5
        );
        assert_eq!(
            "0 -1 -2 -3 -4"
                .parse::<Sequence>()
                .unwrap()
                .previous_value()
                .unwrap(),
            1
        );
        assert_eq!(
            "2 4 7 11"
                .parse::<Sequence>()
                .unwrap()
                .previous_value()
                .unwrap(),
            1
        );
    }
}
